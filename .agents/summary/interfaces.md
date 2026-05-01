# Interfaces

<!-- Generated: 2026-05-01 | tags: api, interfaces, public-surface -->

## Public API Surface

### Chronicle (graph.rs)

```rust
// Construction
Chronicle::from_directory(path: &Path) -> Result<Self, ChronicleError>
Chronicle::from_directory_with_config(path: &Path, config: ValidationConfig) -> Result<Self, ChronicleError>

// Validation
chronicle.validate() -> ValidationReport
chronicle.can_add_event(&Event) -> Result<(), Vec<ValidationError>>

// Query entry points
chronicle.actor(id: &str) -> Option<ActorQuery>
chronicle.event(id: &str) -> Option<EventQuery>
chronicle.place(id: &str) -> Option<PlaceQuery>
chronicle.concept(id: &str) -> Option<ConceptQuery>

// Text retrieval
chronicle.mentions(entity_id: &str) -> Vec<&Account>
chronicle.accounts_of(event_id: &str) -> Vec<&Account>
chronicle.accounts_by(source_id: &str) -> Vec<&Account>
```

### Query Handles

```rust
// ActorQuery
actor.data() -> &Actor
actor.events() -> Vec<&Event>
actor.events_during(range: RangeInclusive<i32>) -> Vec<&Event>
actor.events_at(place_id: &str) -> Vec<&Event>
actor.interactions() -> InteractionResult  // .all(), .people(), .factions()
actor.status_at(year: i32) -> Status

// EventQuery
event.data() -> &Event
event.participants() -> Vec<&Participant>
event.participants_by_role(role: Role) -> Vec<&Participant>
event.location() -> Option<&Place>
event.caused_by() -> Vec<&Event>
event.causal_chain() -> Vec<&Event>       // BFS backward through causes
event.consequences() -> Vec<&Event>        // forward via CausedBy edges
event.state_changes() -> &[EventStateChange]

// PlaceQuery
place.data() -> &Place
place.events() -> Vec<&Event>
place.events_during(range: RangeInclusive<i32>) -> Vec<&Event>
place.actors_present_at(year: i32) -> Vec<&Actor>
place.status_at(year: i32) -> Status

// ConceptQuery
concept.data() -> &Concept
concept.origin_event() -> Option<&Event>
```

### Validation Types

```rust
struct ValidationReport { errors: Vec<ValidationError>, warnings: Vec<ValidationWarning> }
ValidationReport::is_ok() -> bool

enum ValidationError {
    DanglingReference { source_id, target_id, context },
    TemporalViolation { entity_id, event_id, description },
    StateViolation { entity_id, event_id, description },
}

enum ValidationWarning {
    OrphanEntity { entity_id },
    TemporalAmbiguity { description },
}
```

### Configuration

```rust
struct ValidationConfig {
    terminal_statuses: HashSet<Status>,  // default: {Dead, Destroyed, Dissolved}
}
```

## RON Content Format

Entity type is inferred from subdirectory name. Each `.ron` file contains a `Vec<T>`.

```
world/
├── actors/     → Vec<Actor>
├── places/     → Vec<Place>
├── events/     → Vec<Event>
├── concepts/   → Vec<Concept>
└── accounts/   → Vec<Account>
```

### Entity Reference Syntax

Narrative text in accounts uses `{entity_id}` for inline references:

```ron
text: "The {mirror_order} marched on {silica}."
```

References are resolved at load time. Unresolvable references are validation errors.

## Error Handling Pattern

- **Load-time errors** (IO, parse, duplicate ID): `ChronicleError` — returned as `Result::Err`, stops loading
- **Validation errors** (dangling refs, temporal, state): `ValidationError` — collected in `ValidationReport`, never stops processing
- **can_add_event errors**: `Vec<ValidationError>` — all violations collected, returned as `Err`
- **Query misses**: `Option::None` or empty `Vec` — no errors for missing entities
