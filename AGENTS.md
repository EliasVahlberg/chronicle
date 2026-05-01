# AGENTS.md — Chronicle

> Event-centric narrative knowledge graphs with temporal verification.

## Directory Map

```
src/
├── lib.rs          # crate root — re-exports all modules
├── model.rs        # domain types: Entity, Relationship, Status, TimeSpan, ValidationConfig, etc.
├── error.rs        # ChronicleError (load-time errors)
├── graph.rs        # Chronicle struct, RON loading, edge construction, parse_references()
├── query.rs        # typed query API: ActorQuery, EventQuery, PlaceQuery, ConceptQuery
└── validation.rs   # 4 validation passes, ValidationReport, can_add_event()

tests/
├── siege_of_silica.rs      # happy-path integration tests (loading, queries)
├── validation_errors.rs    # negative tests (dangling ref, temporal, state, orphan, duplicate)
├── can_add_event.rs        # insertion verification tests + custom ValidationConfig
├── boundary_conditions.rs  # edge cases (same-year, adjacent-year, spanning timespans)
├── subjective_fragments.rs # account queries by source, fidelity, divergence
└── data/
    ├── world/              # Siege of Silica test cluster (20 entities, 7 RON files)
    └── invalid/            # 5 error scenarios for negative testing

docs/                       # design documents (PROPOSAL, SCOPE, ROADMAP, RESEARCH)
```

## Key Entry Points

| Task | Start here |
|------|-----------|
| Understand the data model | `src/model.rs` — all types, enums, serde attributes |
| Add a new entity type or relationship | `src/model.rs` (type) → `src/graph.rs` (edge construction) → `src/validation.rs` (checks) |
| Add a new query method | `src/query.rs` — follow the pattern of existing query handles |
| Add a new validation rule | `src/validation.rs` — add a pass or extend an existing one |
| Understand temporal verification | `src/validation.rs` — `to_interval()`, `strictly_before()`, `validate_temporal()` |
| See how RON content is authored | `tests/data/world/` — real examples of every entity type |

## Repo-Specific Patterns

**Heterogeneous graph via enums**: `Entity` enum wraps 5 node types, `Relationship` enum wraps 10 edge types. Query methods use `match` or `neighbors_by_edge()` with a predicate to filter by edge type. Do NOT use `neighbors_directed()` alone — it ignores edge types.

**Inclusive TimeSpan → exclusive Allen Interval**: `TimeSpan { start: 10, end: 10 }` becomes `Interval { start: 10, end: 11 }`. The `to_interval()` function handles this. Use `strictly_before()` (precedes ∨ meets) for "happened before" checks, not raw `precedes()`.

**ValidationConfig**: Policy decisions (which statuses are terminal) are in `ValidationConfig`, not hardcoded. Default: Dead, Destroyed, Dissolved. Consumers can override via `from_directory_with_config()`.

**Edge construction is a separate pass**: `build_edges()` collects `(NodeIndex, target_id, Relationship)` tuples first, then resolves. Dangling targets are silently skipped — validation catches them.

**No unwrap() in library code**: All fallible operations return `Result` or `Option`.

## Config Files

| File | Purpose |
|------|---------|
| `Cargo.toml` | 5 dependencies, no dev-deps, edition 2024 |
| `.gitignore` | `/target` and `Cargo.lock` |

No CI config, no linters, no pre-commit hooks. `cargo clippy` and `cargo test` are the quality gates.

## Detailed Documentation

See `.agents/summary/index.md` for the full documentation index. Key files:
- `architecture.md` — design decisions and system structure
- `interfaces.md` — complete public API with signatures
- `data_models.md` — all types and their fields
- `workflows.md` — loading, validation, and query flows

## Custom Instructions
<!-- This section is for human and agent-maintained operational knowledge.
     Add repo-specific conventions, gotchas, and workflow rules here.
     This section is preserved exactly as-is when re-running codebase-summary. -->
