use serde::Deserialize;

mod mostly;
mod platforms;

pub use mostly::{Mostly, MostlyError};
use platforms::Platforms;
use zuwaigani_core::check::CheckKind;
use zuwaigani_core::platform::Platform;

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
    pub fn kind(&self) -> CheckKind {
        match self {
            Self::NotNull {} => CheckKind::NotNull,
            Self::Unique {} => CheckKind::Unique,
            Self::Between { .. } => CheckKind::Between,
            Self::InSet { .. } => CheckKind::InSet,
            Self::MatchRegex { .. } => CheckKind::MatchRegex,
        }
    }

    #[must_use]
    pub fn name(&self) -> &'static str {
        self.kind().name()
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
        self.mostly.map_or(1.0, Mostly::value)
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
