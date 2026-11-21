use zuwaigani::backend;
use zuwaigani::expectation::Expectation;
use zuwaigani_core::check::CheckKind;
use zuwaigani_core::platform::Platform;

#[test]
fn lists_every_check_for_every_platform() {
    let caps = backend::capabilities();
    assert_eq!(caps.platforms, Platform::ALL);
    let kinds: Vec<_> = caps.checks.iter().map(|c| c.check).collect();
    assert_eq!(kinds, CheckKind::ALL);
}

#[test]
fn agrees_with_what_the_suite_check_uses() {
    let caps = backend::capabilities();
    for kind in CheckKind::ALL {
        for platform in Platform::ALL {
            assert_eq!(
                caps.supports(platform, kind),
                backend::supports(platform, kind)
            );
        }
    }
}

#[test]
fn yaml_type_names_map_to_check_kinds() {
    let cases = [
        ("not_null", "", CheckKind::NotNull),
        ("unique", "", CheckKind::Unique),
        ("between", "min: 0\nmax: 1\n", CheckKind::Between),
        ("in_set", "values: [a]\n", CheckKind::InSet),
        ("match_regex", "pattern: x\n", CheckKind::MatchRegex),
    ];
    for (ty, extra, kind) in cases {
        let yaml = format!("type: {ty}\ncolumn: c\nplatform: [polars]\n{extra}");
        let e: Expectation = serde_yaml::from_str(&yaml).expect("valid expectation");
        assert_eq!(e.check().kind(), kind, "type: {ty}");
        assert_eq!(kind.to_string(), ty);
        assert_eq!(e.name(), kind.name());
    }
}
