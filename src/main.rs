use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use zuwaigani::backend;
use zuwaigani::result::ValidationReport;
use zuwaigani::suite::Suite;

#[derive(Parser)]
#[command(version, about = "Great Expectations-style data validation")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate the sources listed in a suite
    Validate {
        /// Suite definition YAML file
        #[arg(short, long)]
        suite: PathBuf,
        /// Write the JSON result to this path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::from(2)
        }
    }
}

fn run(cli: Cli) -> Result<bool> {
    match cli.command {
        Command::Validate { suite, output } => {
            let suite = Suite::from_yaml_file(&suite)?;
            let backends = suite
                .platforms()
                .into_iter()
                .map(|p| backend::open(&suite.source(p)?))
                .collect::<Result<Vec<_>>>()?;
            let report = suite.run(&backends)?;

            print_summary(&report);

            if let Some(path) = output {
                let json = serde_json::to_string_pretty(&report)?;
                std::fs::write(&path, json)
                    .with_context(|| format!("cannot write result: {}", path.display()))?;
                println!("wrote result to {}", path.display());
            }
            Ok(report.success)
        }
    }
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
