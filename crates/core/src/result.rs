use serde::{Deserialize, Serialize};

use crate::platform::Platform;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpectationResult {
    pub platform: Platform,
    pub expectation: String,
    pub column: String,
    pub success: bool,
    /// All rows for `not_null`, non-null rows otherwise.
    pub element_count: u64,
    pub unexpected_count: u64,
    pub unexpected_percent: f64,
    pub mostly: f64,
    pub sample_unexpected: Vec<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationReport {
    pub suite: String,
    pub success: bool,
    pub evaluated: usize,
    pub successful: usize,
    pub results: Vec<ExpectationResult>,
}
