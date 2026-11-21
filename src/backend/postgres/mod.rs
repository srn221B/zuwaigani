pub mod sql;

use std::cell::RefCell;

use ::postgres::types::ToSql;
use ::postgres::{Client, NoTls};
use anyhow::{Context, Result, bail};

use self::sql::Param;
use super::{Backend, Measurement, SAMPLE_LIMIT};
use crate::expectation::{Check, Expectation};
use crate::platform::Platform;

pub struct PostgresBackend {
    client: RefCell<Client>,
    table: String,
}

#[must_use]
pub fn supports(check: &Check) -> bool {
    match check {
        Check::NotNull {}
        | Check::Unique {}
        | Check::Between { .. }
        | Check::InSet { .. }
        | Check::MatchRegex { .. } => true,
    }
}

impl std::fmt::Debug for PostgresBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PostgresBackend")
            .field("table", &self.table)
            .finish_non_exhaustive()
    }
}

impl PostgresBackend {
    pub fn connect(url: &str, table: &str) -> Result<Self> {
        let client = Client::connect(url, NoTls).context("cannot connect to postgres")?;
        Ok(Self {
            client: RefCell::new(client),
            table: table.to_string(),
        })
    }

    fn query(&self, q: &sql::Query) -> Result<Vec<::postgres::Row>> {
        let params: Vec<&(dyn ToSql + Sync)> = q.params.iter().map(to_sql).collect();
        self.client
            .borrow_mut()
            .query(&q.sql, &params)
            .with_context(|| format!("query failed: {}", q.sql))
    }
}

impl Backend for PostgresBackend {
    fn platform(&self) -> Platform {
        Platform::Postgres
    }

    fn columns(&self) -> Result<Vec<String>> {
        let (schema, name) = match self.table.split_once('.') {
            Some((schema, name)) => (Some(schema), name),
            None => (None, self.table.as_str()),
        };
        let rows = self
            .client
            .borrow_mut()
            .query(
                "SELECT column_name::text FROM information_schema.columns \
                 WHERE table_schema = coalesce($1, current_schema()) AND table_name = $2 \
                 ORDER BY ordinal_position",
                &[&schema, &name],
            )
            .context("cannot read columns from information_schema")?;
        if rows.is_empty() {
            bail!("table `{}` not found", self.table);
        }
        rows.iter()
            .map(|r| r.try_get(0).map_err(Into::into))
            .collect()
    }

    fn measure(&self, e: &Expectation) -> Result<Measurement> {
        let rows = self
            .query(&sql::count_query(e, &self.table))
            .with_context(|| format!("failed to run {} (column: {})", e.name(), e.column()))?;
        let row = rows.first().context("count query returned no rows")?;
        let unexpected_count = u64::try_from(row.try_get::<_, i64>("unexpected")?)?;
        let element_count = u64::try_from(row.try_get::<_, i64>("total")?)?;

        let sample_unexpected = if unexpected_count > 0 {
            self.query(&sql::sample_query(e, &self.table, SAMPLE_LIMIT))?
                .iter()
                .map(|r| r.try_get(0).map_err(Into::into))
                .collect::<Result<_>>()?
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

fn to_sql(param: &Param) -> &(dyn ToSql + Sync) {
    match param {
        Param::Float(v) => v,
        Param::Text(v) => v,
        Param::TextArray(v) => v,
    }
}
