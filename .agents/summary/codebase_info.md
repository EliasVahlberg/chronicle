# Codebase Info

<!-- Generated: 2026-05-01 | tags: metadata, build, project -->

## Project

| Field | Value |
|-------|-------|
| Name | chronicle |
| Version | 0.1.0 |
| Edition | Rust 2024 |
| License | MIT |
| Repository | https://github.com/EliasVahlberg/chronicle |
| Published | Not yet on crates.io |

## Keywords

`narrative`, `knowledge-graph`, `world-building`, `temporal`, `lore`

## Categories

`data-structures`, `game-development`

## Entry Points

| Binary/Lib | Path | Purpose |
|------------|------|---------|
| Library | `src/lib.rs` | Crate root — re-exports `error`, `graph`, `model`, `query`, `validation` |

No binary targets. This is a library crate.

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `serde` | 1 (with `derive`) | Serialization/deserialization for all public types |
| `ron` | 0.8 | RON format parsing for authored content |
| `petgraph` | 0.8 (with `serde-1`, `stable_graph`) | Graph data structure and traversal algorithms |
| `allen-intervals` | 0.1 | Allen's Interval Algebra for temporal verification |
| `thiserror` | 2 | Derive macro for error types |

No dev-dependencies. No async, no database, no network.

## Source Structure

```
src/
├── lib.rs          # crate root, module declarations
├── model.rs        # entity types, enums, ValidationConfig
├── error.rs        # ChronicleError enum
├── graph.rs        # Chronicle struct, loading, edge construction
├── query.rs        # typed query API (ActorQuery, EventQuery, PlaceQuery, ConceptQuery)
└── validation.rs   # validation passes, ValidationReport, can_add_event
```

## Test Structure

```
tests/
├── siege_of_silica.rs      # 25 integration tests — loading, queries, validation (happy path)
├── validation_errors.rs    # 5 tests — negative cases (dangling ref, temporal, state, orphan, duplicate)
├── can_add_event.rs        # 12 tests — insertion verification, custom config
├── boundary_conditions.rs  # 9 tests — edge cases (same-year, adjacent-year, spanning timespans)
├── subjective_fragments.rs # 9 tests — accounts by source, fidelity, divergence
└── data/
    ├── world/              # Siege of Silica test cluster (20 entities)
    │   ├── actors/         # factions.ron, characters.ron
    │   ├── places/         # locations.ron
    │   ├── events/         # schism_wars.ron
    │   ├── concepts/       # technologies.ron
    │   └── accounts/       # kaine_durgan.ron, jorik_vane.ron
    └── invalid/            # 5 error scenarios
        ├── dangling_ref/
        ├── duplicate_id/
        ├── orphan/
        ├── state_violation/
        └── temporal_violation/
```
