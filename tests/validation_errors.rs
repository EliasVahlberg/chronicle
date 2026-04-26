use std::path::Path;
use chronicle::graph::Chronicle;
use chronicle::validation::{ValidationError, ValidationWarning};

fn data_path(scenario: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/invalid")
        .join(scenario)
}

// ── Dangling reference ─────────────────────────────────────

#[test]
fn catches_dangling_reference() {
    let c = Chronicle::from_directory(&data_path("dangling_ref")).unwrap();
    let report = c.validate();

    let dangling: Vec<_> = report.errors.iter().filter(|e| {
        matches!(e, ValidationError::DanglingReference { target_id, .. } if target_id == "nonexistent_actor")
    }).collect();

    assert!(!dangling.is_empty(), "should catch dangling reference to nonexistent_actor");
}

// ── Temporal violation (dead actor participates later) ──────

#[test]
fn catches_state_violation_dead_actor() {
    let c = Chronicle::from_directory(&data_path("temporal_violation")).unwrap();
    let report = c.validate();

    let violations: Vec<_> = report.errors.iter().filter(|e| {
        matches!(e, ValidationError::StateViolation { entity_id, event_id, .. }
            if entity_id == "doomed_soldier" && event_id == "later_battle")
    }).collect();

    assert!(!violations.is_empty(), "should catch dead actor participating in later event");
}

// ── State violation (event at destroyed place) ─────────────

#[test]
fn catches_state_violation_destroyed_place() {
    let c = Chronicle::from_directory(&data_path("state_violation")).unwrap();
    let report = c.validate();

    let violations: Vec<_> = report.errors.iter().filter(|e| {
        matches!(e, ValidationError::StateViolation { entity_id, event_id, .. }
            if entity_id == "ruined_city" && event_id == "impossible_market")
    }).collect();

    assert!(!violations.is_empty(), "should catch event at destroyed location");
}

// ── Orphan entity ──────────────────────────────────────────

#[test]
fn catches_orphan_entity() {
    let c = Chronicle::from_directory(&data_path("orphan")).unwrap();
    let report = c.validate();

    let orphans: Vec<_> = report.warnings.iter().filter(|w| {
        matches!(w, ValidationWarning::OrphanEntity { entity_id } if entity_id == "forgotten_hermit")
    }).collect();

    assert!(!orphans.is_empty(), "should warn about orphaned forgotten_hermit");

    // connected_faction should NOT be orphaned (it participates in an event)
    let false_orphans: Vec<_> = report.warnings.iter().filter(|w| {
        matches!(w, ValidationWarning::OrphanEntity { entity_id } if entity_id == "connected_faction")
    }).collect();

    assert!(false_orphans.is_empty(), "connected_faction should not be orphaned");
}

// ── Duplicate ID ───────────────────────────────────────────

#[test]
fn catches_duplicate_id() {
    let result = Chronicle::from_directory(&data_path("duplicate_id"));
    let err = match result {
        Err(e) => e,
        Ok(_) => panic!("should fail to load duplicate IDs"),
    };
    let msg = err.to_string();
    assert!(msg.contains("duplicate_id"), "error should mention the duplicate ID: {msg}");
}
