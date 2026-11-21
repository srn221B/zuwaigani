use serde::{Deserialize, Serialize};

use crate::check::CheckKind;
use crate::platform::Platform;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Capabilities {
    pub platforms: Vec<Platform>,
    pub checks: Vec<CheckSupport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckSupport {
    pub check: CheckKind,
    pub supported_by: Vec<Platform>,
}

impl Capabilities {
    pub fn build(supports: impl Fn(Platform, CheckKind) -> bool) -> Self {
        let checks = CheckKind::ALL
            .into_iter()
            .map(|check| CheckSupport {
                check,
                supported_by: Platform::ALL
                    .into_iter()
                    .filter(|&p| supports(p, check))
                    .collect(),
            })
            .collect();
        Self {
            platforms: Platform::ALL.to_vec(),
            checks,
        }
    }

    #[must_use]
    pub fn supports(&self, platform: Platform, check: CheckKind) -> bool {
        self.checks
            .iter()
            .any(|c| c.check == check && c.supported_by.contains(&platform))
    }
}
