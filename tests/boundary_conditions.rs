//! Boundary-condition tests for validation edge cases (issue #2).

use chronicle::graph::Chronicle;
use chronicle::model::*;
use chronicle::validation::ValidationError;
use std::path::Path;

fn load() -> Chronicle {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/world");
    Chronicle::from_directory(&path).unwrap()
}

fn make_event(id: &str, start: i32, end: i32) -> Event {
    Event {
        id: id.to_owned(),
        name: id.to_owned(),
        event_type: EventType::Political,
        time_span: TimeSpan { start, end },
        location: None,
        caused_by: vec![],
        participants: vec![],
        state_changes: vec![],
        description: String::new(),
    }
}

// ── Same-year events with state changes ────────────────────

#[test]
fn same_year_as_destruction_is_not_a_violation() {
    let c = load();
    // silica destroyed at year 20 by siege_of_silica
    // A new event at year 20 at silica should be OK — same year is ambiguous, not invalid
    let mut event = make_event("concurrent_event", 20, 20);
    event.location = Some("silica".to_owned());
    assert!(c.can_add_event(&event).is_ok());
}

#[test]
fn same_year_as_death_is_not_a_violation() {
    // brother_halix captured at year 16 — not terminal by default
    // But test the pattern: an actor's state changes in the same year they participate
    let c = load();
    let mut event = make_event("same_year_event", 16, 16);
    event.participants.push(Participant {
        actor: "brother_halix".to_owned(),
        role: Role::Participant,
        sentiment: Sentiment::Neutral,
    });
    // Year 16 is when halix was captured — same year, should be fine
    assert!(c.can_add_event(&event).is_ok());
}

// ── Event at place destroyed by that same event ────────────

#[test]
fn siege_of_silica_pattern_is_valid() {
    // The existing data has siege_of_silica at silica which also destroys silica.
    // This should NOT be a validation error — the destruction happens during the event.
    let c = load();
    let report = c.validate();
    let silica_violations: Vec<_> = report.errors.iter().filter(|e| {
        matches!(e, ValidationError::StateViolation { entity_id, .. } if entity_id == "silica")
    }).collect();
    assert!(
        silica_violations.is_empty(),
        "siege_of_silica destroying silica during the event should not be a state violation"
    );
}

// ── Overlapping timespans with causal ordering ─────────────

#[test]
fn overlapping_cause_and_effect_is_valid() {
    let c = load();
    // revelation_of_matthias (15-15) causes great_divide (15-15) — same year
    // This is already in the data and should be valid
    let report = c.validate();
    let causal_violations: Vec<_> = report
        .errors
        .iter()
        .filter(|e| {
            matches!(e, ValidationError::TemporalViolation { entity_id, event_id, .. }
            if entity_id == "great_divide" && event_id == "revelation_of_matthias")
        })
        .collect();
    assert!(
        causal_violations.is_empty(),
        "same-year cause and effect should not be a temporal violation"
    );
}

#[test]
fn can_add_event_with_same_year_cause() {
    let c = load();
    // Propose an event at year 20 caused by siege_of_silica (also year 20)
    let mut event = make_event("aftermath", 20, 20);
    event.caused_by = vec!["siege_of_silica".to_owned()];
    assert!(c.can_add_event(&event).is_ok());
}

// ── Strictly-after state change IS a violation ─────────────

#[test]
fn year_after_destruction_is_a_violation() {
    let c = load();
    // silica destroyed at year 20 — year 21 should fail
    let mut event = make_event("post_destruction", 21, 21);
    event.location = Some("silica".to_owned());
    let errs = c.can_add_event(&event).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e,
        ValidationError::StateViolation { entity_id, .. } if entity_id == "silica"
    )));
}

// ── Multi-role actor across events ─────────────────────────

#[test]
fn actor_in_multiple_roles_across_events_is_valid() {
    // mirror_order participates in 4 events with different roles — should all be fine
    let c = load();
    let report = c.validate();
    let mirror_violations: Vec<_> = report
        .errors
        .iter()
        .filter(|e| {
            matches!(e, ValidationError::StateViolation { entity_id, .. }
            | ValidationError::TemporalViolation { entity_id, .. }
            if entity_id == "mirror_order")
        })
        .collect();
    assert!(mirror_violations.is_empty());
}

// ── Spanning timespan contains point event ─────────────────

#[test]
fn can_add_during_spanning_event() {
    let c = load();
    // purist_uprising spans 15-16. Propose an event at year 15 caused by it.
    // Same-year overlap — should be valid (not "effect precedes cause")
    let mut event = make_event("mid_uprising_event", 15, 15);
    event.caused_by = vec!["purist_uprising".to_owned()];
    assert!(c.can_add_event(&event).is_ok());
}

#[test]
fn effect_strictly_before_spanning_cause_is_violation() {
    let c = load();
    // purist_uprising spans 15-16. Event at year 14 caused by it — violation
    let mut event = make_event("before_uprising", 14, 14);
    event.caused_by = vec!["purist_uprising".to_owned()];
    let errs = c.can_add_event(&event).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e,
        ValidationError::TemporalViolation { description, .. }
        if description.contains("precedes its cause")
    )));
}
