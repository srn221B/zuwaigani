mod common;

use std::path::{Path, PathBuf};

use common::StubBackend;
use zuwaigani::backend::Backend;
use zuwaigani::sources::Source;
use zuwaigani::suite::Suite;
use zuwaigani_core::platform::Platform;

const SOURCES: &str = "sources:
  polars:
    path: data.csv
  postgres:
    url_env: ZUWAIGANI_TEST_UNSET_DATABASE_URL
    table: users
";

fn suite(expectations: &str) -> anyhow::Result<Suite> {
    let yaml = format!("name: t\n{SOURCES}expectations:\n{expectations}");
    Suite::from_yaml_str(&yaml, Path::new("suites"))
}

fn backends(platforms: &[Platform]) -> Vec<Box<dyn Backend>> {
    platforms
        .iter()
        .map(|&p| -> Box<dyn Backend> { Box::new(StubBackend::new(p)) })
        .collect()
}

#[test]
fn runs_each_expectation_on_every_listed_platform() {
    let s = suite(
        "  - type: not_null\n    column: id\n    platform: [polars, postgres]\n  - type: unique\n    column: id\n    platform: [postgres]\n",
    )
    .unwrap();
    assert_eq!(s.platforms(), vec![Platform::Polars, Platform::Postgres]);

    let report = s
        .run(&backends(&[Platform::Polars, Platform::Postgres]))
        .unwrap();
    let ran: Vec<_> = report
        .results
        .iter()
        .map(|r| (r.expectation.as_str(), r.platform))
        .collect();
    assert_eq!(
        ran,
        vec![
            ("expect_column_values_to_not_be_null", Platform::Polars),
            ("expect_column_values_to_not_be_null", Platform::Postgres),
            ("expect_column_values_to_be_unique", Platform::Postgres),
        ]
    );
}

#[test]
fn errors_when_a_platform_has_no_source() {
    let yaml = "name: t\nsources:\n  polars:\n    path: data.csv\nexpectations:\n  - type: not_null\n    column: id\n    platform: [postgres]\n";
    let err = Suite::from_yaml_str(yaml, Path::new("")).unwrap_err();
    assert!(err.to_string().contains("sources.postgres"), "{err}");
}

#[test]
fn errors_on_empty_suite() {
    assert!(suite(" []\n").is_err());
}

#[test]
fn errors_on_missing_column() {
    let s = suite("  - type: not_null\n    column: nope\n    platform: [polars]\n").unwrap();
    assert!(s.run(&backends(&[Platform::Polars])).is_err());
}

#[test]
fn resolves_file_paths_from_the_suite_directory() {
    let s = suite("  - type: not_null\n    column: id\n    platform: [polars]\n").unwrap();
    let Source::File(path) = s.source(Platform::Polars).unwrap() else {
        panic!("expected a file source");
    };
    assert_eq!(path, PathBuf::from("suites/data.csv"));
}

#[test]
fn errors_when_the_url_variable_is_unset() {
    let s = suite("  - type: not_null\n    column: id\n    platform: [postgres]\n").unwrap();
    let err = s.source(Platform::Postgres).unwrap_err();
    assert!(
        err.to_string()
            .contains("ZUWAIGANI_TEST_UNSET_DATABASE_URL"),
        "{err}"
    );
}
