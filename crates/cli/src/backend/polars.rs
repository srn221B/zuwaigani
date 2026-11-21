use std::path::Path;

use ::polars::prelude::*;
use anyhow::{Result, anyhow, bail};

use super::{Backend, Measurement, SAMPLE_LIMIT};
use crate::expectation::{Check, Expectation};
use zuwaigani_core::check::CheckKind;
use zuwaigani_core::platform::Platform;

pub struct PolarsBackend {
    lf: LazyFrame,
}

#[must_use]
pub fn supports(kind: CheckKind) -> bool {
    match kind {
        CheckKind::NotNull
        | CheckKind::Unique
        | CheckKind::Between
        | CheckKind::InSet
        | CheckKind::MatchRegex => true,
    }
}

impl std::fmt::Debug for PolarsBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PolarsBackend").finish_non_exhaustive()
    }
}

impl PolarsBackend {
    pub fn open(path: &Path) -> Result<Self> {
        if !path.is_file() {
            bail!("data file not found: {}", path.display());
        }
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(str::to_ascii_lowercase);

        let path_str = path
            .to_str()
            .ok_or_else(|| anyhow!("non-UTF-8 paths are not supported: {}", path.display()))?;
        let lf = match ext.as_deref() {
            Some("csv") => LazyCsvReader::new(PlRefPath::new(path_str))
                .with_has_header(true)
                .finish(),
            Some("parquet" | "pq") => {
                LazyFrame::scan_parquet(PlRefPath::new(path_str), ScanArgsParquet::default())
            }
            _ => bail!("unsupported file extension: {}", path.display()),
        };
        // `polars::prelude::PolarsContext` also has `with_context`; call anyhow's explicitly.
        let lf =
            anyhow::Context::with_context(lf, || format!("cannot open data: {}", path.display()))?;
        Ok(Self { lf })
    }

    fn sample_unexpected(&self, e: &Expectation) -> Result<Vec<Option<String>>> {
        let df = self
            .lf
            .clone()
            .filter(unexpected_expr(e))
            .select([col(e.column()).cast(DataType::String)])
            .limit(SAMPLE_LIMIT)
            .collect()?;
        let ca = df.column(e.column())?.str()?;
        Ok(ca.iter().map(|v| v.map(str::to_string)).collect())
    }
}

impl Backend for PolarsBackend {
    fn platform(&self) -> Platform {
        Platform::Polars
    }

    fn columns(&self) -> Result<Vec<String>> {
        let schema = self.lf.clone().collect_schema()?;
        Ok(schema.iter_names().map(ToString::to_string).collect())
    }

    fn measure(&self, e: &Expectation) -> Result<Measurement> {
        let collected = self
            .lf
            .clone()
            .select([
                unexpected_expr(e)
                    .sum()
                    .cast(DataType::UInt64)
                    .alias("unexpected"),
                denominator_expr(e).cast(DataType::UInt64).alias("total"),
            ])
            .collect();
        let counts = anyhow::Context::with_context(collected, || {
            format!("failed to run {} (column: {})", e.name(), e.column())
        })?;

        let unexpected_count = first_u64(&counts, "unexpected")?;
        let element_count = first_u64(&counts, "total")?;

        let sample_unexpected = if unexpected_count > 0 {
            self.sample_unexpected(e)?
        } else {
            Vec::new()
        };

        Ok(Measurement {
            element_count,
            unexpected_count,
            sample_unexpected,
        })
    }
}

fn unexpected_expr(e: &Expectation) -> Expr {
    let c = col(e.column());
    match e.check() {
        Check::NotNull {} => c.is_null(),
        Check::Unique {} => c.clone().is_not_null().and(c.is_duplicated()),
        Check::Between { min, max } => c.clone().is_not_null().and(
            c.is_between(lit(*min), lit(*max), ClosedInterval::Both)
                .not(),
        ),
        Check::InSet { values } => {
            let set = Series::new("set".into(), values.as_slice());
            let s = c.cast(DataType::String);
            s.clone()
                .is_not_null()
                .and(s.is_in(lit(set).implode(false), false).not())
        }
        Check::MatchRegex { pattern } => {
            let s = c.cast(DataType::String);
            s.clone()
                .is_not_null()
                .and(s.str().contains(lit(pattern.as_str()), true).not())
        }
    }
}

fn denominator_expr(e: &Expectation) -> Expr {
    match e.check() {
        Check::NotNull {} => len(),
        _ => col(e.column()).is_not_null().sum(),
    }
}

fn first_u64(df: &DataFrame, name: &str) -> Result<u64> {
    df.column(name)?
        .u64()?
        .get(0)
        .ok_or_else(|| anyhow!("aggregate `{name}` is empty"))
}
