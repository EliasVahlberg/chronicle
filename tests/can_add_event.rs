use std::collections::HashSet;
use std::path::Path;
use chronicle::graph::Chronicle;
use chronicle::model::*;
use chronicle::validation::ValidationError;

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

// ── Valid insertion ─────────────────────────────────────────

#[test]
fn can_add_valid_event() {
    let c = load();
    let mut event = make_event("new_treaty", 25, 25);
    event.participants.push(Participant {
        actor: "iron_covenant".to_owned(),
        role: Role::Participant,
        sentiment: Sentiment::Neutral,
    });
    event.location = Some("central_steppe".to_owned());
    event.caused_by = vec!["siege_of_silica".to_owned()];

    assert!(c.can_add_event(&event).is_ok());
}

// ── Duplicate ID ───────────────────────────────────────────

#[test]
fn rejects_duplicate_id() {
    let c = load();
    let event = make_event("siege_of_silica", 30, 30);
    let errs = c.can_add_event(&event).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e, ValidationError::DanglingReference { context, .. }
        if context.contains("same ID"))));
}

// ── Dangling references ────────────────────────────────────

#[test]
fn rejects_dangling_participant() {
    let c = load();
    let mut event = make_event("new_event", 25, 25);
    event.participants.push(Participant {
        actor: "ghost_faction".to_owned(),
        role: Role::Attacker,
        sentiment: Sentiment::Determined,
    });
    let errs = c.can_add_event(&event).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e,
        ValidationError::DanglingReference { target_id, context, .. }
        if target_id == "ghost_faction" && context == "participant"
    )));
}

#[test]
fn rejects_dangling_location() {
    let c = load();
    let mut event = make_event("new_event", 25, 25);
    event.location = Some("nonexistent_place".to_owned());
    let errs = c.can_add_event(&event).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e,
        ValidationError::DanglingReference { target_id, .. } if target_id == "nonexistent_place"
    )));
}

#[test]
fn rejects_dangling_caused_by() {
    let c = load();
    let mut event = make_event("new_event", 25, 25);
    event.caused_by = vec!["nonexistent_event".to_owned()];
    let errs = c.can_add_event(&event).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e,
        ValidationError::DanglingReference { target_id, .. } if target_id == "nonexistent_event"
    )));
}

// ── Temporal violations ────────────────────────────────────

#[test]
fn rejects_effect_before_cause() {
    let c = load();
    // siege_of_silica is at year 20; propose an event at year 5 caused by it
    let mut event = make_event("impossible_prequel", 5, 5);
    event.caused_by = vec!["siege_of_silica".to_owned()];
    let errs = c.can_add_event(&event).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e,
        ValidationError::TemporalViolation { description, .. }
        if description.contains("precedes its cause")
    )));
}

// ── State violations ───────────────────────────────────────

#[test]
fn rejects_dead_actor_participation() {
    let c = load();
    // silica was destroyed at year 20 by siege_of_silica
    // brother_halix was captured at year 16 by battle_of_broken_glass
    // Captured is NOT terminal by default, so halix should be fine
    // But silica IS destroyed (terminal), so using it as location should fail
    let mut event = make_event("post_siege_market", 25, 25);
    event.location = Some("silica".to_owned());
    let errs = c.can_add_event(&event).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e,
        ValidationError::StateViolation { entity_id, description, .. }
        if entity_id == "silica" && description.contains("Destroyed")
    )));
}

#[test]
fn allows_captured_actor_by_default() {
    let c = load();
    // Captured is NOT terminal by default
    let mut event = make_event("prison_event", 25, 25);
    event.participants.push(Participant {
        actor: "brother_halix".to_owned(),
        role: Role::Participant,
        sentiment: Sentiment::Defiant,
    });
    assert!(c.can_add_event(&event).is_ok());
}

// ── Custom ValidationConfig ────────────────────────────────

#[test]
fn custom_config_captured_is_terminal() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/world");
    let config = ValidationConfig {
        terminal_statuses: [Status::Dead, Status::Destroyed, Status::Dissolved, Status::Captured].into(),
    };
    let c = Chronicle::from_directory_with_config(&path, config).unwrap();

    // Now Captured IS terminal — brother_halix was captured at year 16
    let mut event = make_event("prison_event", 25, 25);
    event.participants.push(Participant {
        actor: "brother_halix".to_owned(),
        role: Role::Participant,
        sentiment: Sentiment::Defiant,
    });
    let errs = c.can_add_event(&event).unwrap_err();
    assert!(errs.iter().any(|e| matches!(e,
        ValidationError::StateViolation { entity_id, description, .. }
        if entity_id == "brother_halix" && description.contains("Captured")
    )));
}

#[test]
fn custom_config_nothing_terminal() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/world");
    let config = ValidationConfig {
        terminal_statuses: HashSet::new(),
    };
    let c = Chronicle::from_directory_with_config(&path, config).unwrap();

    // With no terminal statuses, even destroyed places are fine
    let mut event = make_event("ghost_market", 25, 25);
    event.location = Some("silica".to_owned());
    assert!(c.can_add_event(&event).is_ok());
}

// ── Multiple errors collected ──────────────────────────────

#[test]
fn collects_all_errors_not_just_first() {
    let c = load();
    let mut event = make_event("total_mess", 25, 25);
    event.location = Some("nonexistent_place".to_owned());
    event.participants.push(Participant {
        actor: "ghost_faction".to_owned(),
        role: Role::Attacker,
        sentiment: Sentiment::Determined,
    });
    event.caused_by = vec!["nonexistent_cause".to_owned()];

    let errs = c.can_add_event(&event).unwrap_err();
    // Should have at least 3 errors: dangling location, participant, and caused_by
    assert!(errs.len() >= 3, "expected at least 3 errors, got {}", errs.len());
}

// ── Error messages are actionable ──────────────────────────

#[test]
fn error_messages_contain_full_context() {
    let c = load();
    let mut event = make_event("post_siege_gathering", 25, 25);
    event.location = Some("silica".to_owned());

    let errs = c.can_add_event(&event).unwrap_err();
    let msg = errs[0].to_string();

    // Should mention: the entity, the status, the causing event, years
    assert!(msg.contains("silica"), "should mention entity: {msg}");
    assert!(msg.contains("Destroyed"), "should mention status: {msg}");
    assert!(msg.contains("siege_of_silica"), "should mention causing event: {msg}");
    assert!(msg.contains("20"), "should mention year: {msg}");
    assert!(msg.contains("25"), "should mention proposed year: {msg}");
}
