use zuwaigani_core::capabilities::Capabilities;
use zuwaigani_core::check::CheckKind;
use zuwaigani_core::platform::Platform;

#[test]
fn lists_every_check_in_order() {
    let caps = Capabilities::build(|_, _| true);
    let kinds: Vec<_> = caps.checks.iter().map(|c| c.check).collect();
    assert_eq!(kinds, CheckKind::ALL);
}

#[test]
fn records_which_platforms_support_which_check() {
    let caps = Capabilities::build(|p, c| !(p == Platform::Polars && c == CheckKind::MatchRegex));
    assert!(!caps.supports(Platform::Polars, CheckKind::MatchRegex));
    assert!(caps.supports(Platform::Postgres, CheckKind::MatchRegex));
    assert!(caps.supports(Platform::Polars, CheckKind::NotNull));
}

#[test]
fn json_uses_the_names_written_in_yaml() {
    let caps = Capabilities::build(|p, _| p == Platform::Postgres);
    let json = serde_json::to_value(&caps).expect("serializes");
    assert_eq!(json["platforms"], serde_json::json!(["polars", "postgres"]));
    assert_eq!(json["checks"][0]["check"], "not_null");
    assert_eq!(
        json["checks"][0]["supported_by"],
        serde_json::json!(["postgres"])
    );
    let back: Capabilities = serde_json::from_value(json).expect("deserializes");
    assert_eq!(back, caps);
}

#[test]
fn display_matches_the_serde_name() {
    for kind in CheckKind::ALL {
        let json = serde_json::to_value(kind).expect("serializes");
        assert_eq!(json, kind.to_string());
    }
}
