# chronicle
[[github]](https://github.com/EliasVahlberg/chronicle)
[[crates.io]](https://crates.io/crates/chronicle-graph)
[[docs.rs]](https://docs.rs/chronicle-graph)

Event-centric narrative knowledge graphs with temporal verification.

Load authored world-building content into a typed knowledge graph, validate it
for consistency, and query it. Events are the connective tissue between actors,
places, and time — adapted from [CIDOC CRM](https://cidoc-crm.org/) (ISO 21127).
Temporal verification uses [Allen's Interval Algebra](https://en.wikipedia.org/wiki/Allen%27s_interval_algebra).

```toml
[dependencies]
chronicle-graph = "0.1"
```

## Example

```rust
use chronicle::graph::Chronicle;
use chronicle::model::*;

let graph = Chronicle::from_directory("world/".as_ref())?;
let report = graph.validate();

// Typed queries
let kaine = graph.actor("kaine_durgan").unwrap();
let events = kaine.events();
let contacts = kaine.interactions().factions();
let chain = graph.event("siege_of_silica").unwrap().causal_chain();
let status = graph.place("silica").unwrap().status_at(25); // Destroyed

// Verify a proposed event against the existing graph
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
    Ok(()) => println!("consistent"),
    Err(errors) => {
        for e in &errors { eprintln!("{e}"); }
        // "location 'silica' has status Destroyed as of 'siege_of_silica' (year 20),
        //  cannot host proposed event 'new_battle' (year 25)"
    }
}
```

## What it does

World data is authored as RON files in a directory structure. Entity type is
inferred from the subdirectory name (`actors/*.ron` → `Vec<Actor>`, etc.).
Narrative text uses `{entity_id}` references resolved at load time.

The graph validates: referential integrity (all IDs resolve), temporal
consistency (lifespans contain events, causes precede effects), and state
tracking (dead actors don't participate, destroyed places don't host events).
Which statuses are terminal is configurable via `ValidationConfig`.

Accounts model unreliable narrators — each has a source and a fidelity rating
(Canonical, Partial, Biased, Fabricated, etc.). Two conflicting accounts of the
same event coexist with queryable divergence.

## Key concepts

- **Event-centric model** — all relationships pass through events with participants, roles, sentiment, location, causality, and state changes
- **Allen's Interval Algebra** — 13 temporal relations on discrete integer years; `strictly_before` = precedes ∨ meets
- **ValidationConfig** — policy object controlling which statuses are terminal (default: Dead, Destroyed, Dissolved)
- **can_add_event** — pure pre-flight check against the existing graph, no mutation
- **Subjective fragments** — accounts with fidelity ratings, queryable by source, event, and entity mentions

## Status

v0.1.0 — graph loading, validation, typed query API, insertion verification, subjective fragments. See `docs/ROADMAP.md` for the full plan.

## License

MIT
