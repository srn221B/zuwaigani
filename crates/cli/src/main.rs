mod commands;

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Result;
use clap::{Parser, Subcommand};

use commands::platforms::Format;

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
    /// Show which checks each platform supports
    Platforms {
        /// Print JSON instead of a table
        #[arg(long)]
        json: bool,
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
        Command::Validate { suite, output } => commands::validate::run(&suite, output.as_deref()),
        Command::Platforms { json } => {
            let format = if json { Format::Json } else { Format::Table };
            commands::platforms::run(format)
        }
    }
}
