use std::path::Path;

use zuwaigani::backend;
use zuwaigani_core::capabilities::Capabilities;

#[test]
fn frontend_capabilities_json_is_up_to_date() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../frontend/data/capabilities.json");
    let text = std::fs::read_to_string(&path).expect("frontend/data/capabilities.json exists");
    let committed: Capabilities =
        serde_json::from_str(&text).expect("capabilities.json matches the core types");
    assert_eq!(
        committed,
        backend::capabilities(),
        "frontend/data/capabilities.json is stale; run `make capabilities`"
    );
}
