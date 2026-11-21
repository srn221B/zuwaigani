use std::error::Error;
use std::fmt;

use serde::Deserialize;

use zuwaigani_core::platform::Platform;

#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "Vec<Platform>")]
pub(super) struct Platforms(Vec<Platform>);

impl Platforms {
    pub(super) fn as_slice(&self) -> &[Platform] {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum PlatformsError {
    Empty,
    Duplicate(Platform),
}

impl fmt::Display for PlatformsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("platform must list at least one platform"),
            Self::Duplicate(p) => write!(f, "platform lists `{p}` more than once"),
        }
    }
}

impl Error for PlatformsError {}

impl TryFrom<Vec<Platform>> for Platforms {
    type Error = PlatformsError;

    fn try_from(platforms: Vec<Platform>) -> Result<Self, PlatformsError> {
        if platforms.is_empty() {
            return Err(PlatformsError::Empty);
        }
        for (i, p) in platforms.iter().enumerate() {
            if platforms[..i].contains(p) {
                return Err(PlatformsError::Duplicate(*p));
            }
        }
        Ok(Self(platforms))
    }
}
