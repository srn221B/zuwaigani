use serde::Serialize;

use crate::backend::Measurement;
use crate::expectation::Expectation;
use crate::platform::Platform;

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct ValidationReport {
    pub suite: String,
    pub success: bool,
    pub evaluated: usize,
    pub successful: usize,
    pub results: Vec<ExpectationResult>,
}

impl ExpectationResult {
    #[must_use]
    pub fn judge(platform: Platform, expectation: &Expectation, m: Measurement) -> Self {
        let unexpected_percent = percent(m.unexpected_count, m.element_count);
        let mostly = expectation.mostly();
        // Tolerate float error so that mostly = 1.0 with zero unexpected rows still passes.
        let success = (100.0 - unexpected_percent) / 100.0 >= mostly - f64::EPSILON;

        Self {
            platform,
            expectation: expectation.name().to_string(),
            column: expectation.column().to_string(),
            success,
            element_count: m.element_count,
            unexpected_count: m.unexpected_count,
            unexpected_percent,
            mostly,
            sample_unexpected: m.sample_unexpected,
        }
    }
}

#[expect(
    clippy::cast_precision_loss,
    clippy::as_conversions,
    reason = "row counts stay exact in f64 below 2^52"
)]
fn percent(part: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        part as f64 / total as f64 * 100.0
    }
}
