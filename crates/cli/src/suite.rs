use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::backend::{self, Backend};
use crate::expectation::Expectation;
use crate::result::{self, ValidationReport};
use crate::sources::{Source, Sources};
use zuwaigani_core::platform::Platform;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Suite {
    pub name: String,
    #[serde(default)]
    pub sources: Sources,
    pub expectations: Vec<Expectation>,
    #[serde(skip)]
    base_dir: PathBuf,
}

impl Suite {
    pub fn from_yaml_file(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("cannot read suite file: {}", path.display()))?;
        let base_dir = path.parent().unwrap_or_else(|| Path::new(""));
        Self::from_yaml_str(&text, base_dir)
            .with_context(|| format!("invalid suite: {}", path.display()))
    }

    pub fn from_yaml_str(text: &str, base_dir: &Path) -> Result<Self> {
        let mut suite: Self = serde_yaml::from_str(text)?;
        suite.base_dir = base_dir.to_path_buf();
        suite.check()?;
        Ok(suite)
    }

    fn check(&self) -> Result<()> {
        if self.expectations.is_empty() {
            bail!("suite has no expectations");
        }
        for e in &self.expectations {
            for &p in e.platforms() {
                if !backend::supports(p, e.check()) {
                    bail!(
                        "{} (column: {}) is not supported on {p}",
                        e.name(),
                        e.column()
                    );
                }
                if !self.sources.is_configured(p) {
                    bail!(
                        "{} (column: {}) runs on {p}, but `sources.{p}` is not configured",
                        e.name(),
                        e.column()
                    );
                }
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn platforms(&self) -> Vec<Platform> {
        let mut platforms: Vec<Platform> = self
            .expectations
            .iter()
            .flat_map(|e| e.platforms().iter().copied())
            .collect();
        platforms.sort();
        platforms.dedup();
        platforms
    }

    pub fn source(&self, platform: Platform) -> Result<Source> {
        self.sources.resolve(platform, &self.base_dir)
    }

    pub fn run(&self, backends: &[Box<dyn Backend>]) -> Result<ValidationReport> {
        let backend_for = |p: Platform| -> Result<&dyn Backend> {
            backends
                .iter()
                .find(|b| b.platform() == p)
                .map(AsRef::as_ref)
                .with_context(|| format!("no backend opened for {p}"))
        };

        for p in self.platforms() {
            let columns = backend_for(p)?.columns()?;
            for e in self.expectations.iter().filter(|e| e.runs_on(p)) {
                if !columns.iter().any(|c| c == e.column()) {
                    bail!(
                        "column `{}` not found on {p} ({}). available columns: {}",
                        e.column(),
                        e.name(),
                        columns.join(", ")
                    );
                }
            }
        }

        let mut results = Vec::new();
        for e in &self.expectations {
            for &p in e.platforms() {
                let backend = backend_for(p)?;
                results.push(result::judge(p, e, backend.measure(e)?));
            }
        }

        let successful = results.iter().filter(|r| r.success).count();
        Ok(ValidationReport {
            suite: self.name.clone(),
            success: successful == results.len(),
            evaluated: results.len(),
            successful,
            results,
        })
    }
}
