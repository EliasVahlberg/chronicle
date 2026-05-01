# Narrative Knowledge Graphs — Scope

> Structured world-building as queryable, verifiable entity-relation graphs.
>
> Companion to [PROPOSAL.md](PROPOSAL.md) (why this exists) and [INITIAL_RESEARCH.md](INITIAL_RESEARCH.md) (theoretical foundations).

## What It Does

A Rust crate that loads authored narrative content into a typed, event-centric knowledge graph, validates it for consistency, and provides a query interface for traversal and subgraph extraction.

The core operation: load a set of RON files defining entities, events, relationships, and narrative text with embedded references. Build an in-memory graph. Validate temporal, spatial, and referential consistency. Answer queries.

```rust
let graph = Chronicle::from_directory("world/".as_ref())?;
let report = graph.validate();

// Typed queries
let contacts = graph.actor("brother_qon").unwrap().interactions();
let chain = graph.event("schism_wars").unwrap().causal_chain();

// Verification queries
let ok = graph.can_add_event(&proposed_event)?;
```

## What It Doesn't Do

- **Parse natural language.** Content is hand-authored with explicit entity references. No NLP, no NER.
- **Generate narrative text.** The crate stores and queries structured lore. Prose generation is the consumer's responsibility.
- **Render or visualize.** Output is structured data. Visualization (if wanted) is a separate tool.
- **Require a database.** The graph lives in memory. No Neo4j, no SQLite, no external dependencies beyond serde/ron.
- **Know about any specific game.** The crate defines a general narrative graph model. Saltglass-steppe is the first consumer, not a hard dependency.

## Data Model

### Entity Types

Adapted from CIDOC CRM (simplified for game use — see [PROPOSAL.md](PROPOSAL.md) for the relationship to CIDOC CRM):

| Type | Examples | Key Properties |
|------|----------|----------------|
| Actor | Characters, factions, deities, organizations | id, name, type, status (alive/dead/dissolved), lifespan |
| Place | Regions, cities, dungeons, landmarks | id, name, type, coordinates (optional), status |
| Event | Battles, discoveries, deaths, migrations | id, name, type, time_span, cause, participants, location |
| Concept | Religions, technologies, artifacts, laws | id, name, type, origin_event |

TimePeriod (named eras spanning multiple events) is deferred. Events carry their own `time_span`.

**Note:** The crate is published as `chronicle-graph` on crates.io (`cargo add chronicle-graph`). The library name remains `chronicle` for imports (`use chronicle::graph::Chronicle`).

### Embedded References in Narrative Text

Narrative prose uses entity IDs as inline references rather than raw names. This keeps text human-readable while making every mention a resolvable, queryable graph reference.

```ron
(
    id: "siege_of_silica_account_01",
    source: "brother_qon",           // subjective fragment owner
    fidelity: Partial,               // omissions, no fabrication
    text: "I was there when {mirror_order} marched on {silica_citadel}. \
           {commander_vael} led the assault — though {elder_reth} will tell you \
           it was {faction_salt_merchants} who struck first. That was the same \
           season {the_glass_storm_of_847} shattered the eastern wall.",
)
```

Reference syntax:
- `{entity_id}` — reference to an Actor, Place, Concept, or Event node

The `[$time_ref]` syntax for inline time references is deferred — `time_span` on events is sufficient for now.

At load time, all references are resolved against the graph. Unresolvable references are validation errors. This means:
- Every entity mentioned in prose *must* exist as a node
- Queries can find all narrative passages that mention a given entity
- Renaming an entity updates the node; references stay stable via ID

### Relationships

All relationships pass through events (event-centric model):

```ron
(
    id: "siege_of_silica",
    type: Battle,
    time_span: (start: 847, end: 847),
    location: "silica_citadel",
    caused_by: ["schism_wars"],
    participants: [
        (actor: "commander_vael", role: Attacker, sentiment: Triumphant),
        (actor: "mirror_order", role: Attacker, sentiment: Triumphant),
        (actor: "silica_garrison", role: Defender, sentiment: Devastating),
        (actor: "brother_qon", role: Witness, sentiment: Sorrowful),
    ],
    state_changes: [
        (entity: "silica_citadel", change: StatusChange(Damaged)),
        (entity: "silica_garrison", change: StatusChange(Dissolved)),
    ],
)
```

### Subjective Fragments

Each narrative account belongs to a source with a fidelity rating:

| Fidelity | Meaning | Example Source |
|----------|---------|----------------|
| Canonical | Matches objective graph exactly | Archive-drone with intact records |
| Partial | Omissions but no fabrication | Eyewitness with incomplete view |
| Distorted | Genuine misremembering | Elderly NPC recounting old events |
| Biased | Deliberate spin, selective truth | Faction propagandist |
| Fabricated | Intentional lies | Trickster NPC, corrupted record |
| Corrupted | Damaged/degraded record | Storm-damaged archive-drone |

The objective graph is the union of all Canonical-fidelity content. Subjective fragments are queryable by source, fidelity, and divergence from objective.

## Validation

On load (and on insertion), the graph validates:

### Referential Integrity
- All `{entity_id}` references in text resolve to existing nodes
- All relationship targets (caused_by, participants, location) resolve
- No orphaned nodes (entities that nothing references and that reference nothing)

### Temporal Consistency (Allen's Interval Algebra)
- An actor's participation in an event falls within their lifespan
- Causal chains are temporally ordered (cause precedes effect)
- State changes are respected (dead actors don't participate in later events)

### Spatial Plausibility (optional, requires geography data)
- An actor's sequential event participation is spatially feasible given travel time between locations

### Output
```rust
pub struct ValidationReport {
    pub errors: Vec<ValidationError>,    // hard failures (dangling refs, dead actor participating)
    pub warnings: Vec<ValidationWarning>, // soft issues (orphaned nodes, temporal gaps)
}
```

## Query Interface

Queries are typed method calls on graph handles, not natural language. The API is simple because the event metadata is robust — rich, well-typed event records make traversal straightforward.

### Actor Queries
```rust
graph.actor("brother_qon").interactions()           // all co-participants across all events
graph.actor("brother_qon").interactions().people()   // filter to Actor type
graph.actor("brother_qon").interactions().factions()  // filter to faction type
graph.actor("brother_qon").events()                  // all events involving this actor
graph.actor("brother_qon").events_during(800..900)   // filtered by time range
graph.actor("brother_qon").events_at("silica_citadel") // filtered by place
graph.actor("brother_qon").status_at(850)            // alive/dead/status at a point in time
```

### Event Queries
```rust
graph.event("siege_of_silica").participants()        // all actors involved
graph.event("siege_of_silica").participants_by_role(Attacker) // filtered by role
graph.event("siege_of_silica").location()            // where it happened
graph.event("siege_of_silica").time_span()           // when
graph.event("siege_of_silica").caused_by()           // direct causes
graph.event("siege_of_silica").causal_chain()        // full chain backward
graph.event("siege_of_silica").consequences()        // events caused by this one
graph.event("siege_of_silica").state_changes()       // what changed in the world
```

### Place Queries
```rust
graph.place("silica_citadel").events()               // all events here
graph.place("silica_citadel").events_during(800..900) // filtered by time
graph.place("silica_citadel").actors_present_at(847)  // who was here at this time
graph.place("silica_citadel").status_at(850)          // intact/damaged/destroyed
```

### Subgraph Extraction
```rust
graph.subgraph("schism_wars", depth: 2)             // everything within 2 hops
graph.subgraph_filtered("schism_wars", depth: 2, types: &[Actor, Place]) // filtered
```

### Verification
```rust
graph.can_add_event(&proposed_event)                 // would this be consistent?
graph.validate()                                     // full consistency report
```

### Text Retrieval
```rust
graph.mentions("silica_citadel")                     // all narrative passages referencing this entity
graph.accounts_of("siege_of_silica")                 // all subjective accounts, with fidelity
graph.accounts_by("kaine_durgan")                    // all accounts authored by this source
```

## File Organization

```
world/
├── actors/
│   ├── characters.ron      # individual characters
│   ├── factions.ron         # factions and organizations
│   └── creatures.ron        # creature types
├── places/
│   ├── regions.ron          # major regions
│   └── landmarks.ron        # specific locations
├── events/
│   ├── schism_wars.ron      # event cluster
│   ├── heliograph.ron       # event cluster
│   └── ...
├── concepts/
│   └── technologies.ron     # religions, artifacts, laws
└── accounts/
    ├── brother_qon.ron      # subjective fragment
    ├── archive_drone_7.ron  # high-fidelity fragment
    └── ...
```

Each file contains a list of typed entries. The crate loads the entire directory, builds the graph, and validates.

## API Shape

```rust
// Load
let graph = NarrativeGraph::from_directory("world/")?;

// Validate
let report = graph.validate();
for error in &report.errors {
    eprintln!("{}", error);
}

// Typed queries — no natural language, just method chains
let allies = graph.actor("brother_qon").interactions().people();
let history = graph.place("silica_citadel").events_during(800..900);
let chain = graph.event("siege_of_silica").causal_chain();
let status = graph.actor("commander_vael").status_at(850);

// Verify before adding
let proposed = Event { id: "new_event", time_span: (860, 860), .. };
match graph.can_add(&proposed) {
    Ok(()) => graph.insert(proposed)?,
    Err(conflicts) => eprintln!("Cannot add: {:?}", conflicts),
}

// Text retrieval
let passages = graph.mentions("silica_citadel");
let accounts = graph.accounts_of("siege_of_silica");
```

## Dependencies

Minimal:
- `serde` + `ron` — deserialization
- `petgraph` — graph data structure (`StableGraph` with enum node/edge types)
- No async, no database, no network, no GPU

## Relationship to saltglass-steppe

The crate is a standalone published library. Saltglass-steppe depends on it the same way it depends on terrain-forge:

- Lore content lives in `data/world/` as RON files
- Build step or startup validates the graph
- Game systems query the graph at runtime (spawn tables, quest triggers, NPC dialogue)
- AI agents reference the graph's validation output instead of reading raw prose documents

## Open Questions

- **petgraph vs custom**: petgraph is mature but generic. A custom graph optimized for this domain's query patterns might be simpler and faster. Decide during prototyping.
- **Incremental validation**: Full validation on every load, or incremental on insertion? Start with full, optimize later.
- **Serialization of query results**: Return node references, cloned subgraphs, or a cursor-based API?
- **How much CIDOC CRM to adopt**: The full standard is enormous. We need the minimal subset that gives us event-centric modeling + temporal verification.

### Resolved

| Question | Decision |
|----------|----------|
| Crate name | `chronicle` — [github.com/EliasVahlberg/chronicle](https://github.com/EliasVahlberg/chronicle) |
| petgraph vs custom | petgraph `StableGraph` — get algorithms for free, wrap with typed API |
| File format | RON — prototyped with real lore, ergonomic enough |
| `[$time_ref]` syntax in prose | Deferred — `time_span` on events is sufficient for now |
| TimePeriod entity type | Deferred to after Phase 1 — events carry their own time_span |
