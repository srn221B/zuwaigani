use anyhow::Result;
use zuwaigani::backend::{Backend, Measurement};
use zuwaigani::expectation::Expectation;
use zuwaigani_core::platform::Platform;

pub(crate) struct StubBackend {
    pub platform: Platform,
    pub columns: Vec<String>,
    pub element_count: u64,
    pub unexpected_count: u64,
}

impl StubBackend {
    pub(crate) fn new(platform: Platform) -> Self {
        Self {
            platform,
            columns: vec!["id".into()],
            element_count: 3,
            unexpected_count: 0,
        }
    }
}

impl Backend for StubBackend {
    fn platform(&self) -> Platform {
        self.platform
    }

    fn columns(&self) -> Result<Vec<String>> {
        Ok(self.columns.clone())
    }

    fn measure(&self, _: &Expectation) -> Result<Measurement> {
        Ok(Measurement {
            element_count: self.element_count,
            unexpected_count: self.unexpected_count,
            sample_unexpected: Vec::new(),
        })
    }
}
