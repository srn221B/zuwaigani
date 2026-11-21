use zuwaigani::backend::Measurement;
use zuwaigani::expectation::Expectation;
use zuwaigani::result::judge;
use zuwaigani_core::platform::Platform;

fn success(element_count: u64, unexpected_count: u64, mostly: Option<&str>) -> bool {
    let yaml = format!(
        "type: not_null\ncolumn: id\nplatform: [polars]\n{}",
        mostly.map_or(String::new(), |m| format!("mostly: {m}"))
    );
    let e: Expectation = serde_yaml::from_str(&yaml).expect("test YAML is valid");
    let m = Measurement {
        element_count,
        unexpected_count,
        sample_unexpected: Vec::new(),
    };
    judge(Platform::Polars, &e, m).success
}

#[test]
fn strict_by_default() {
    assert!(success(8, 0, None));
    assert!(!success(8, 1, None));
}

#[test]
fn mostly_threshold_is_inclusive() {
    assert!(success(5, 1, Some("0.8")));
    assert!(!success(8, 2, Some("0.8")));
}

#[test]
fn no_target_rows_is_success() {
    assert!(success(0, 0, None));
}
