pub mod polars;
pub mod postgres;

use anyhow::Result;

use crate::expectation::Expectation;
use crate::sources::Source;
use zuwaigani_core::capabilities::Capabilities;
use zuwaigani_core::check::CheckKind;
use zuwaigani_core::platform::Platform;

pub const SAMPLE_LIMIT: u32 = 5;

#[derive(Debug)]
pub struct Measurement {
    pub element_count: u64,
    pub unexpected_count: u64,
    pub sample_unexpected: Vec<Option<String>>,
}

pub trait Backend {
    fn platform(&self) -> Platform;

    fn columns(&self) -> Result<Vec<String>>;

    fn measure(&self, expectation: &Expectation) -> Result<Measurement>;
}

pub fn open(source: &Source) -> Result<Box<dyn Backend>> {
    Ok(match source {
        Source::File(path) => Box::new(polars::PolarsBackend::open(path)?),
        Source::Postgres { url, table } => {
            Box::new(postgres::PostgresBackend::connect(url, table)?)
        }
    })
}

#[must_use]
pub fn supports(platform: Platform, kind: CheckKind) -> bool {
    match platform {
        Platform::Polars => polars::supports(kind),
        Platform::Postgres => postgres::supports(kind),
    }
}

#[must_use]
pub fn capabilities() -> Capabilities {
    Capabilities::build(supports)
}
