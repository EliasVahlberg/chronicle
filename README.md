# chronicle

Event-centric narrative knowledge graphs with temporal verification.

A Rust crate for structured world-building: load authored narrative content into a typed, event-centric knowledge graph, validate it for consistency, and query it.

## The Problem

World-building for narrative-heavy games accumulates as disconnected documents that drift into inconsistency. Characters contradict themselves across files, timelines break, and AI-generated content introduces plausible-sounding details that conflict with established canon. At scale, nobody can hold the world in their head.

## The Approach

Replace disconnected prose with structured entity-relation graphs. Every piece of world-building — character, faction, location, event — is a typed node with explicit, validated relationships. Events are the connective tissue (adapted from [CIDOC CRM](https://cidoc-crm.org/), ISO 21127). Temporal consistency uses [Allen's Interval Algebra](https://en.wikipedia.org/wiki/Allen%27s_interval_algebra).

## Usage

```rust
use chronicle::graph::Chronicle;
use chronicle::model::*;

// Load a world from RON files
let graph = Chronicle::from_directory("world/".as_ref())?;

// Validate consistency
let report = graph.validate();
for error in &report.errors {
    eprintln!("{error}");
}

// Typed queries
let kaine = graph.actor("kaine_durgan").unwrap();
let events = kaine.events();
let contacts = kaine.interactions().people();
let chain = graph.event("siege_of_silica").unwrap().causal_chain();
let status = graph.place("silica").unwrap().status_at(25); // Destroyed

// Check if a proposed event is consistent
let proposed = Event {
    id: "new_battle".into(),
    name: "New Battle".into(),
    event_type: EventType::Battle,
    time_span: TimeSpan { start: 25, end: 25 },
    location: Some("silica".into()),
    caused_by: vec![],
    participants: vec![],
    state_changes: vec![],
    description: String::new(),
};
match graph.can_add_event(&proposed) {
    Ok(()) => println!("Event is consistent"),
    Err(errors) => {
        for e in &errors {
            eprintln!("{e}");
            // "location 'silica' has status Destroyed as of 'siege_of_silica' (year 20),
            //  cannot host proposed event 'new_battle' (year 25)"
        }
    }
}

// Subjective accounts with fidelity ratings
let accounts = graph.accounts_of("siege_of_silica");
for account in accounts {
    println!("{}: {:?} fidelity", account.source, account.fidelity);
}
```

## Content Format

World data is authored as RON files in a directory structure. Entity type is inferred from the subdirectory name:

```
world/
├── actors/         # Vec<Actor> — characters, factions, organizations
├── places/         # Vec<Place> — settlements, regions, landmarks
├── events/         # Vec<Event> — battles, discoveries, political events
├── concepts/       # Vec<Concept> — religions, technologies, artifacts
└── accounts/       # Vec<Account> — subjective narrative text
```

Narrative text uses `{entity_id}` references:

```ron
(
    id: "kaine_siege_journal",
    source: "kaine_durgan",
    fidelity: Biased,
    event_refs: ["siege_of_silica"],
    text: "We burned {silica}. The {mirror_order} call it a massacre. We call it sanitation.",
)
```

## Features

- **Event-centric model**: All relationships pass through events (who, where, when, why)
- **Temporal verification**: Allen's Interval Algebra catches impossible timelines
- **State tracking**: Dead actors can't participate in future events; destroyed places can't host them
- **Configurable policies**: `ValidationConfig` controls which statuses are terminal
- **Subjective fragments**: Model unreliable narrators with fidelity ratings (Canonical, Partial, Biased, etc.)
- **Typed queries**: Method chains over structured data, not natural language
- **Insertion verification**: `can_add_event()` checks proposed records against the existing graph

## Custom Validation Config

```rust
use chronicle::model::{ValidationConfig, Status};

let config = ValidationConfig {
    terminal_statuses: [Status::Dead, Status::Destroyed, Status::Dissolved, Status::Captured].into(),
};
let graph = Chronicle::from_directory_with_config("world/".as_ref(), config)?;
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `petgraph` | Graph structure and traversal algorithms |
| `allen-intervals` | Temporal consistency verification (Allen's Interval Algebra) |
| `serde` + `ron` | Deserialization of hand-authored content |
| `thiserror` | Error types |

No async, no database, no network, no GPU.

## Status

Phases 1–4 complete. Not yet published on crates.io.

- ✅ Graph model, loading, edge construction
- ✅ Validation (referential, temporal, state, orphan detection)
- ✅ Typed query API (actor, event, place, concept queries)
- ✅ Insertion verification (`can_add_event` with `ValidationConfig`)
- ✅ Subjective fragments (accounts with fidelity ratings)
- 🔲 Saltglass-steppe integration (first consumer)

## License

MIT
