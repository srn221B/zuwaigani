use zuwaigani::expectation::Expectation;
use zuwaigani_core::platform::Platform;

fn parse(yaml: &str) -> Result<Expectation, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

#[test]
fn rejects_unknown_field() {
    assert!(parse("type: not_null\ncolumn: id\nplatform: [polars]\nmostley: 0.9").is_err());
}

#[test]
fn rejects_unknown_field_on_a_check_with_values() {
    assert!(
        parse("type: between\ncolumn: age\nplatform: [polars]\nmin: 0\nmax: 1\nmaxx: 2").is_err()
    );
}

#[test]
fn rejects_unknown_platform() {
    assert!(parse("type: not_null\ncolumn: id\nplatform: [polras]").is_err());
}

#[test]
fn requires_platform() {
    assert!(parse("type: not_null\ncolumn: id").is_err());
}

#[test]
fn rejects_empty_platform() {
    assert!(parse("type: not_null\ncolumn: id\nplatform: []").is_err());
}

#[test]
fn rejects_duplicate_platform() {
    assert!(parse("type: not_null\ncolumn: id\nplatform: [polars, polars]").is_err());
}

#[test]
fn rejects_mostly_out_of_range() {
    for bad in ["95", "-0.1", "1.01", ".nan"] {
        let yaml = format!("type: not_null\ncolumn: id\nplatform: [polars]\nmostly: {bad}");
        assert!(parse(&yaml).is_err(), "mostly: {bad}");
    }
}

#[test]
fn accepts_mostly_bounds() {
    for ok in ["0", "0.0", "1", "1.0"] {
        let yaml = format!("type: not_null\ncolumn: id\nplatform: [polars]\nmostly: {ok}");
        assert!(parse(&yaml).is_ok(), "mostly: {ok}");
    }
}

#[test]
fn mostly_defaults_to_one() {
    let e = parse("type: not_null\ncolumn: id\nplatform: [polars]").unwrap();
    assert!((e.mostly() - 1.0).abs() < f64::EPSILON);
}

#[test]
fn runs_only_on_listed_platforms() {
    let e = parse("type: not_null\ncolumn: id\nplatform: [postgres]").unwrap();
    assert!(!e.runs_on(Platform::Polars));
    assert!(e.runs_on(Platform::Postgres));
}
