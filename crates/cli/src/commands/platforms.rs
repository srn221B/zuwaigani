use anyhow::Result;
use zuwaigani::backend;
use zuwaigani_core::capabilities::Capabilities;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Format {
    Table,
    Json,
}

pub(crate) fn run(format: Format) -> Result<bool> {
    let capabilities = backend::capabilities();
    match format {
        Format::Table => print_capabilities(&capabilities),
        Format::Json => println!("{}", serde_json::to_string_pretty(&capabilities)?),
    }
    Ok(true)
}

fn print_capabilities(capabilities: &Capabilities) {
    let width = capabilities
        .checks
        .iter()
        .map(|c| c.check.to_string().len())
        .max()
        .unwrap_or(0);
    let header: Vec<String> = capabilities
        .platforms
        .iter()
        .map(ToString::to_string)
        .collect();
    println!("{:<width$}  {}", "check", header.join("  "));
    for c in &capabilities.checks {
        let cells: Vec<String> = capabilities
            .platforms
            .iter()
            .zip(&header)
            .map(|(p, h)| {
                let mark = if c.supported_by.contains(p) {
                    "yes"
                } else {
                    "no"
                };
                format!("{mark:<w$}", w = h.len())
            })
            .collect();
        println!(
            "{:<width$}  {}",
            c.check.to_string(),
            cells.join("  ").trim_end()
        );
    }
}
