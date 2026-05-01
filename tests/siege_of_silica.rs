use std::path::Path;
use chronicle::graph::Chronicle;
use chronicle::model::*;

fn load() -> Chronicle {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/world");
    Chronicle::from_directory(&path).expect("should load without error")
}

// ── Loading & validation ───────────────────────────────────

#[test]
fn load_siege_of_silica() {
    let c = load();
    // 4 factions + 5 characters + 3 places + 5 events + 2 accounts + 1 concept = 20 nodes
    assert_eq!(c.graph.node_count(), 20);
    assert!(c.index.contains_key("kaine_durgan"));
    assert!(c.index.contains_key("siege_of_silica"));
    assert!(c.index.contains_key("silica"));
    assert!(c.index.contains_key("kaine_durgan_siege_journal"));
    assert!(c.graph.edge_count() > 0);
}

#[test]
fn validate_siege_of_silica_clean() {
    let c = load();
    let report = c.validate();
    for e in &report.errors {
        eprintln!("ERROR: {e}");
    }
    assert!(report.errors.is_empty(), "expected no validation errors");
}

// ── Actor queries ──────────────────────────────────────────

#[test]
fn actor_query_kaine_durgan_events() {
    let c = load();
    let kaine = c.actor("kaine_durgan").expect("kaine_durgan should exist");
    assert_eq!(kaine.data().name, "Commander Kaine Durgan");

    let events = kaine.events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].id, "siege_of_silica");
}

#[test]
fn actor_query_brother_halix_events() {
    let c = load();
    let halix = c.actor("brother_halix").expect("brother_halix should exist");
    let events = halix.events();
    // Halix participates in purist_uprising and battle_of_broken_glass
    assert_eq!(events.len(), 2);
    let ids: Vec<&str> = events.iter().map(|e| e.id.as_str()).collect();
    assert!(ids.contains(&"purist_uprising"));
    assert!(ids.contains(&"battle_of_broken_glass"));
}

#[test]
fn actor_query_events_during_range() {
    let c = load();
    let mirror = c.actor("mirror_order").expect("mirror_order should exist");
    // mirror_order participates in: great_divide(15), purist_uprising(15-16),
    // battle_of_broken_glass(16), siege_of_silica(20)
    let early = mirror.events_during(15..=16);
    let ids: Vec<&str> = early.iter().map(|e| e.id.as_str()).collect();
    assert!(ids.contains(&"great_divide"));
    assert!(ids.contains(&"purist_uprising"));
    assert!(ids.contains(&"battle_of_broken_glass"));
    assert!(!ids.contains(&"siege_of_silica")); // year 20, outside range
}

#[test]
fn actor_query_events_at_place() {
    let c = load();
    let mirror = c.actor("mirror_order").unwrap();
    let at_silica = mirror.events_at("silica");
    assert_eq!(at_silica.len(), 1);
    assert_eq!(at_silica[0].id, "siege_of_silica");
}

#[test]
fn actor_interactions_kaine_durgan() {
    // "Who has Kaine Durgan interacted with?"
    let c = load();
    let kaine = c.actor("kaine_durgan").unwrap();
    let interactions = kaine.interactions();

    let people = interactions.people();
    assert!(people.is_empty()); // no other characters in siege_of_silica

    let factions = interactions.factions();
    let faction_ids: Vec<&str> = factions.iter().map(|a| a.id.as_str()).collect();
    assert!(faction_ids.contains(&"iron_covenant"));
    assert!(faction_ids.contains(&"mirror_order"));
}

#[test]
fn actor_interactions_ressa_vane() {
    let c = load();
    let ressa = c.actor("ressa_vane").unwrap();
    let interactions = ressa.interactions();

    let people = interactions.people();
    let people_ids: Vec<&str> = people.iter().map(|a| a.id.as_str()).collect();
    assert!(people_ids.contains(&"brother_halix"));

    let factions = interactions.factions();
    let faction_ids: Vec<&str> = factions.iter().map(|a| a.id.as_str()).collect();
    assert!(faction_ids.contains(&"sand_engineers"));
    assert!(faction_ids.contains(&"mirror_order"));
}

#[test]
fn actor_status_at_time() {
    let c = load();
    // brother_halix is Captured since battle_of_broken_glass (year 16)
    let halix = c.actor("brother_halix").unwrap();
    assert_eq!(halix.status_at(15), Status::Active); // before capture
    assert_eq!(halix.status_at(16), Status::Captured); // year of capture
    assert_eq!(halix.status_at(25), Status::Captured); // still captured later
}

// ── Event queries ──────────────────────────────────────────

#[test]
fn event_query_siege_participants() {
    // "What factions were involved at Silica?"
    let c = load();
    let siege = c.event("siege_of_silica").unwrap();

    let attackers = siege.participants_by_role(Role::Attacker);
    let attacker_ids: Vec<&str> = attackers.iter().map(|p| p.actor.as_str()).collect();
    assert!(attacker_ids.contains(&"kaine_durgan"));
    assert!(attacker_ids.contains(&"iron_covenant"));

    let defenders = siege.participants_by_role(Role::Defender);
    assert_eq!(defenders.len(), 1);
    assert_eq!(defenders[0].actor, "mirror_order");
}

#[test]
fn event_query_location() {
    let c = load();
    let siege = c.event("siege_of_silica").unwrap();
    let loc = siege.location().expect("siege should have a location");
    assert_eq!(loc.id, "silica");
}

#[test]
fn event_query_causal_chain() {
    // "What events caused the Siege of Silica?"
    let c = load();
    let siege = c.event("siege_of_silica").unwrap();

    // Direct cause
    let direct = siege.caused_by();
    assert_eq!(direct.len(), 1);
    assert_eq!(direct[0].id, "purist_uprising");

    // Full chain: purist_uprising <- great_divide <- revelation_of_matthias
    let chain = siege.causal_chain();
    let chain_ids: Vec<&str> = chain.iter().map(|e| e.id.as_str()).collect();
    assert!(chain_ids.contains(&"purist_uprising"));
    assert!(chain_ids.contains(&"great_divide"));
    assert!(chain_ids.contains(&"revelation_of_matthias"));
    assert_eq!(chain.len(), 3);
}

#[test]
fn event_query_consequences() {
    let c = load();
    let uprising = c.event("purist_uprising").unwrap();
    let consequences = uprising.consequences();
    let ids: Vec<&str> = consequences.iter().map(|e| e.id.as_str()).collect();
    // purist_uprising caused both battle_of_broken_glass and siege_of_silica
    assert!(ids.contains(&"battle_of_broken_glass"));
    assert!(ids.contains(&"siege_of_silica"));
}

#[test]
fn event_query_state_changes() {
    let c = load();
    let siege = c.event("siege_of_silica").unwrap();
    let changes = siege.state_changes();
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].entity, "silica");
    assert_eq!(changes[0].change, StateChange::StatusChange(Status::Destroyed));
}

// ── Place queries ──────────────────────────────────────────

#[test]
fn place_query_events_at_silica() {
    let c = load();
    let silica = c.place("silica").unwrap();
    let events = silica.events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].id, "siege_of_silica");
}

#[test]
fn place_query_events_during() {
    let c = load();
    let havens = c.place("havens_rest").unwrap();
    let events = havens.events_during(15..=17);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].id, "battle_of_broken_glass");
}

#[test]
fn place_query_actors_present_at() {
    let c = load();
    let havens = c.place("havens_rest").unwrap();
    let actors = havens.actors_present_at(16);
    let ids: Vec<&str> = actors.iter().map(|a| a.id.as_str()).collect();
    assert!(ids.contains(&"ressa_vane"));
    assert!(ids.contains(&"sand_engineers"));
    assert!(ids.contains(&"brother_halix"));
    assert!(ids.contains(&"mirror_order"));
    assert_eq!(actors.len(), 4);
}

#[test]
fn place_status_at_time() {
    let c = load();
    let silica = c.place("silica").unwrap();
    assert_eq!(silica.status_at(19), Status::Active); // before siege
    assert_eq!(silica.status_at(20), Status::Destroyed); // year of siege
    assert_eq!(silica.status_at(25), Status::Destroyed); // still destroyed
}

// ── Text retrieval ─────────────────────────────────────────

#[test]
fn mentions_silica() {
    // "What accounts mention silica?"
    let c = load();
    let accounts = c.mentions("silica");
    assert_eq!(accounts.len(), 2); // kaine + jorik both mention silica
}

#[test]
fn mentions_mirror_order() {
    let c = load();
    let accounts = c.mentions("mirror_order");
    assert_eq!(accounts.len(), 2); // kaine + jorik both mention mirror_order
}

#[test]
fn accounts_of_siege() {
    let c = load();
    let accounts = c.accounts_of("siege_of_silica");
    assert_eq!(accounts.len(), 2); // kaine (Biased) + jorik (Partial)
    let sources: Vec<&str> = accounts.iter().map(|a| a.source.as_str()).collect();
    assert!(sources.contains(&"kaine_durgan"));
    assert!(sources.contains(&"jorik_vane"));
}

// ── Nonexistent entity queries return None ─────────────────

#[test]
fn query_nonexistent_returns_none() {
    let c = load();
    assert!(c.actor("nonexistent").is_none());
    assert!(c.event("nonexistent").is_none());
    assert!(c.place("nonexistent").is_none());
    assert!(c.mentions("nonexistent").is_empty());
    assert!(c.accounts_of("nonexistent").is_empty());
}

// ── Cross-type query: wrong type returns None ──────────────

#[test]
fn query_wrong_type_returns_none() {
    let c = load();
    // silica is a Place, not an Actor
    assert!(c.actor("silica").is_none());
    // kaine_durgan is an Actor, not an Event
    assert!(c.event("kaine_durgan").is_none());
}

// ── Concept queries ────────────────────────────────────────

#[test]
fn concept_loads_and_validates() {
    let c = load();
    assert!(c.index.contains_key("null_field_technology"));
    // origin_event reference should resolve (no validation errors for it)
    let report = c.validate();
    let concept_errors: Vec<_> = report.errors.iter().filter(|e| {
        matches!(e, chronicle::validation::ValidationError::DanglingReference { source_id, .. }
            if source_id == "null_field_technology")
    }).collect();
    assert!(concept_errors.is_empty());
}
