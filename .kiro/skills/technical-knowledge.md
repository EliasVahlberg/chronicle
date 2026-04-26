---
name: technical-knowledge
description: Chronicle's Rust architecture, dependency stack, code structure, and implementation patterns. Use when writing code, reviewing implementations, or making technical decisions.
---

# Chronicle — Technical Knowledge

## Dependency Stack

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
ron = "0.8"
petgraph = { version = "0.8", features = ["serde-1", "stable_graph"] }
allen-intervals = "0.1"
thiserror = "2"
```

No async, no database, no network, no GPU.

## Code Structure

```
src/
├── lib.rs          # crate root, re-exports
├── model.rs        # core types: Actor, Place, Event, Concept, Account, enums
└── error.rs        # ChronicleError enum
```

### Core Types (model.rs)

- `EntityId` = `String` — stable string identifiers
- `TimeSpan { start: i32, end: i32 }` — inclusive integer years, point events have start == end
- `Status` — Active, Dead, Destroyed, Dissolved, Evacuated, Captured, Transformed, Corrupted, Unknown
- `Sentiment` — per-participant feeling about an event (Triumphant, Devastating, Sorrowful, etc.)
- `Role` — participant's role in an event (Attacker, Defender, Leader, Witness, Discoverer, etc.)
- `Fidelity` — account reliability (Canonical, Partial, Distorted, Biased, Fabricated, Corrupted)
- `StateChange` — StatusChange, Founded, AffiliationAdded, AffiliationRemoved
- `Participant { actor: EntityId, role: Role, sentiment: Sentiment }`
- `EventStateChange { entity: EntityId, change: StateChange }`
- Entity structs: `Actor`, `Place`, `Event`, `Concept`, `Account` — all `Serialize + Deserialize`

### Error Types (error.rs)

`ChronicleError`: EntityNotFound, DuplicateId, DanglingReference, TemporalViolation, StateViolation, Parse, Io

## Implementation Patterns

- **petgraph StableGraph** with enum node type (`Entity::Actor(..)`, `Entity::Event(..)`, etc.) and enum edge type for relationships
- **HashMap<EntityId, NodeIndex>** for O(1) reference resolution
- **allen-intervals** for temporal checks: wrap TimeSpan into `NonEmpty<Interval>`, use trait methods (Precedes, Contains, etc.)
- **Serde derives** on all public types — RON files deserialize directly into structs
- **Directory-based loading**: walk `world/` directory, detect file category from subdirectory, deserialize lists of entities
- **Validation as a separate pass** after graph construction — collect all errors, don't fail on first

## RON Schema

Entity references in narrative text use `{entity_id}` syntax. Parsed with simple regex/string scanning at load time, resolved against the entity HashMap.

Events are the central node type — they connect actors to places and times:
```ron
(
    id: "siege_of_silica",
    type: Battle,
    time_span: (start: 20, end: 20),
    location: "silica",
    caused_by: ["purist_uprising"],
    participants: [
        (actor: "kaine_durgan", role: Attacker, sentiment: Dutiful),
        (actor: "mirror_order", role: Defender, sentiment: Desperate),
    ],
    state_changes: [
        (entity: "silica", change: StatusChange(Destroyed)),
    ],
)
```

## What Needs Building (Phase 1)

1. Graph builder: load RON → resolve refs → build StableGraph
2. Reference resolver: parse `{entity_id}` from Account text
3. Validation passes: referential integrity, temporal consistency, state tracking
4. ValidationReport output struct
5. Test with Siege of Silica cluster (prototype exists in planning docs)
