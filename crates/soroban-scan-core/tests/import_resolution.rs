//! Aliased imports are resolved for detector context (#41).

use std::path::PathBuf;

use soroban_scan_core::config::ScanConfig;
use soroban_scan_core::discovery::DiscoveryOptions;
use soroban_scan_core::engine;
use soroban_scan_core::project;
use soroban_scan_core::registry::RuleRegistry;

fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/projects/aliased-imports")
}

#[test]
fn aliased_client_constructor_is_detected_by_cross_contract_rule() {
    let loaded = project::load(&fixture(), &DiscoveryOptions::default())
        .expect("aliased-imports fixture must load");
    let outcome = engine::scan(
        &loaded,
        &RuleRegistry::with_default_rules(),
        &ScanConfig::default(),
    );

    assert!(
        outcome
            .findings
            .iter()
            .any(|finding| finding.rule_id == "SS-002"),
        "SS-002 should resolve `C::new` through `use crate::token::Client as C`"
    );
}
