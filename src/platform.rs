use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Polars,
    Postgres,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Polars => "polars",
            Self::Postgres => "postgres",
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "Vec<Platform>")]
pub struct Platforms(Vec<Platform>);

impl Platforms {
    #[must_use]
    pub fn as_slice(&self) -> &[Platform] {
        &self.0
    }
}

impl TryFrom<Vec<Platform>> for Platforms {
    type Error = String;

    fn try_from(platforms: Vec<Platform>) -> Result<Self, String> {
        if platforms.is_empty() {
            return Err("platform must list at least one platform".to_string());
        }
        for (i, p) in platforms.iter().enumerate() {
            if platforms[..i].contains(p) {
                return Err(format!("platform lists `{p}` more than once"));
            }
        }
        Ok(Self(platforms))
    }
}
