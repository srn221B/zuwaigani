use serde::Deserialize;

use zuwaigani_core::platform::Platform;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(try_from = "f64")]
pub struct Mostly(f64);

impl TryFrom<f64> for Mostly {
    type Error = String;

    fn try_from(v: f64) -> Result<Self, String> {
        if (0.0..=1.0).contains(&v) {
            Ok(Self(v))
        } else {
            Err(format!("mostly must be between 0.0 and 1.0: {v}"))
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "Vec<Platform>")]
struct Platforms(Vec<Platform>);

impl Platforms {
    fn as_slice(&self) -> &[Platform] {
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

#[derive(Debug, Deserialize)]
pub struct Expectation {
    column: String,
    mostly: Option<Mostly>,
    platform: Platforms,
    #[serde(flatten)]
    check: Check,
}

/// Every check except `NotNull` ignores nulls.
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Check {
    // `{}` rather than unit variants: under `flatten`, unit variants accept unknown keys.
    NotNull {},
    Unique {},
    Between {
        min: f64,
        max: f64,
    },
    /// Values are compared as strings.
    InSet {
        values: Vec<String>,
    },
    /// Values are compared as strings.
    MatchRegex {
        pattern: String,
    },
}

impl Check {
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::NotNull {} => "expect_column_values_to_not_be_null",
            Self::Unique {} => "expect_column_values_to_be_unique",
            Self::Between { .. } => "expect_column_values_to_be_between",
            Self::InSet { .. } => "expect_column_values_to_be_in_set",
            Self::MatchRegex { .. } => "expect_column_values_to_match_regex",
        }
    }
}

impl Expectation {
    #[must_use]
    pub fn check(&self) -> &Check {
        &self.check
    }

    #[must_use]
    pub fn name(&self) -> &'static str {
        self.check.name()
    }

    #[must_use]
    pub fn column(&self) -> &str {
        &self.column
    }

    #[must_use]
    pub fn mostly(&self) -> f64 {
        self.mostly.map_or(1.0, |m| m.0)
    }

    #[must_use]
    pub fn platforms(&self) -> &[Platform] {
        self.platform.as_slice()
    }

    #[must_use]
    pub fn runs_on(&self, target: Platform) -> bool {
        self.platforms().contains(&target)
    }
}
