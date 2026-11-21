use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

use zuwaigani_core::platform::Platform;

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sources {
    pub polars: Option<FileSource>,
    pub postgres: Option<PostgresSource>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileSource {
    /// Relative to the directory containing the suite file.
    pub path: PathBuf,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PostgresSource {
    /// Name of the environment variable holding the connection URL.
    pub url_env: String,
    pub table: String,
}

pub enum Source {
    File(PathBuf),
    Postgres { url: String, table: String },
}

impl std::fmt::Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(path) => f.debug_tuple("File").field(path).finish(),
            Self::Postgres { table, .. } => f
                .debug_struct("Postgres")
                .field("table", table)
                .finish_non_exhaustive(),
        }
    }
}

impl Sources {
    #[must_use]
    pub fn is_configured(&self, platform: Platform) -> bool {
        match platform {
            Platform::Polars => self.polars.is_some(),
            Platform::Postgres => self.postgres.is_some(),
        }
    }

    pub fn resolve(&self, platform: Platform, base_dir: &Path) -> Result<Source> {
        let missing = || format!("`sources.{platform}` is not configured");
        match platform {
            Platform::Polars => {
                let s = self.polars.as_ref().with_context(missing)?;
                Ok(Source::File(base_dir.join(&s.path)))
            }
            Platform::Postgres => {
                let s = self.postgres.as_ref().with_context(missing)?;
                let url = std::env::var(&s.url_env)
                    .with_context(|| format!("environment variable `{}` is not set", s.url_env))?;
                Ok(Source::Postgres {
                    url,
                    table: s.table.clone(),
                })
            }
        }
    }
}
