# chronicle

Event-centric narrative knowledge graphs with temporal verification.

A Rust crate for structured world-building: load authored narrative content into a typed, event-centric knowledge graph, validate it for consistency, and query it.

## Status

Early development. Not yet published on crates.io.

## The Problem

World-building for narrative-heavy games accumulates as disconnected documents that drift into inconsistency. Characters contradict themselves across files, timelines break, and AI-generated content introduces plausible-sounding details that conflict with established canon.

## The Approach

Replace disconnected prose with structured entity-relation graphs. Every piece of world-building — character, faction, location, event — is a typed node with explicit, validated relationships. Events are the glue (adapted from CIDOC CRM). Temporal consistency uses Allen's Interval Algebra.

```rust
let graph = Chronicle::from_directory("world/")?;
let report = graph.validate();

// Typed queries — no natural language
let contacts = graph.actor("brother_qon").interactions().people();
let chain = graph.event("siege_of_silica").causal_chain();
let can_add = graph.can_add(&proposed_event)?;
```

## Key Features (planned)

- **Event-centric model**: All relationships pass through events (who, where, when, why)
- **Temporal verification**: Allen's Interval Algebra catches impossible timelines
- **Subjective fragments**: Model unreliable narrators — NPCs, biased accounts, corrupted records
- **Typed queries**: Method chains, not natural language
- **RON format**: Hand-authorable structured content with `{entity_id}` references in prose

## Dependencies

- `petgraph` — graph structure and traversal algorithms
- `allen-intervals` — temporal consistency verification
- `serde` + `ron` — deserialization
- `thiserror` — error types

## License

MIT
