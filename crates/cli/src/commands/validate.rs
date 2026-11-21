use std::path::Path;

use anyhow::{Context, Result};
use zuwaigani::backend;
use zuwaigani::result::ValidationReport;
use zuwaigani::suite::Suite;

pub(crate) fn run(suite: &Path, output: Option<&Path>) -> Result<bool> {
    let suite = Suite::from_yaml_file(suite)?;
    let backends = suite
        .platforms()
        .into_iter()
        .map(|p| backend::open(&suite.source(p)?))
        .collect::<Result<Vec<_>>>()?;
    let report = suite.run(&backends)?;

    print_summary(&report);

    if let Some(path) = output {
        let json = serde_json::to_string_pretty(&report)?;
        std::fs::write(path, json)
            .with_context(|| format!("cannot write result: {}", path.display()))?;
        println!("wrote result to {}", path.display());
    }
    Ok(report.success)
}

fn print_summary(report: &ValidationReport) {
    println!("suite: {}", report.suite);
    for r in &report.results {
        let mark = if r.success { "**OK**" } else { "**NG**" };
        println!(
            "  {mark} {:<9} {:<40} {:<12} unexpected {}/{} ({:.2}%)",
            r.platform.to_string(),
            r.expectation,
            r.column,
            r.unexpected_count,
            r.element_count,
            r.unexpected_percent
        );
        if !r.success && !r.sample_unexpected.is_empty() {
            let samples: Vec<&str> = r
                .sample_unexpected
                .iter()
                .map(|v| v.as_deref().unwrap_or("null"))
                .collect();
            println!("      example: {}", samples.join(", "));
        }
    }
    println!(
        "{}  ({}/{} passed)",
        if report.success { "PASSED" } else { "FAILED" },
        report.successful,
        report.evaluated
    );
}
