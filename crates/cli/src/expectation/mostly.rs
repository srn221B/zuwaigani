use std::error::Error;
use std::fmt;

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deserialize)]
#[serde(try_from = "f64")]
pub struct Mostly(f64);

impl Mostly {
    #[must_use]
    pub fn value(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MostlyError(f64);

impl fmt::Display for MostlyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mostly must be between 0.0 and 1.0: {}", self.0)
    }
}

impl Error for MostlyError {}

impl TryFrom<f64> for Mostly {
    type Error = MostlyError;

    fn try_from(v: f64) -> Result<Self, MostlyError> {
        if (0.0..=1.0).contains(&v) {
            Ok(Self(v))
        } else {
            Err(MostlyError(v))
        }
    }
}
