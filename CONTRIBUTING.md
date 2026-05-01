# Contributing to Chronicle

## Development Setup

```bash
git clone git@github.com:EliasVahlberg/chronicle.git
cd chronicle
cargo build
cargo test
```

Requires Rust edition 2024 (stable toolchain). No additional tools or system dependencies.

## Quality Gates

Before committing:

```bash
cargo clippy    # zero warnings required
cargo test      # all tests must pass
```

No CI is configured — these are run locally.

## Project Structure

```
src/
├── model.rs        # domain types — start here for data model changes
├── graph.rs        # Chronicle struct, loading, edge construction
├── query.rs        # typed query API
├── validation.rs   # validation passes + can_add_event
└── error.rs        # error types
```

See `AGENTS.md` for a detailed entry-point guide.

## How to Add Things

### New Entity Type or Relationship

1. Add the type/enum variant to `src/model.rs`
2. Add edge construction in `graph.rs::build_edges()`
3. Add referential validation in `validation.rs::validate_referential()`
4. Add a query handle in `query.rs` if needed
5. Add test data to `tests/data/world/` and write tests

### New Validation Rule

1. Add a function in `validation.rs` following the pattern of existing passes
2. Wire it into `validate()` and/or `can_add_event()`
3. Add negative test data in `tests/data/invalid/` and a test in `tests/validation_errors.rs`

### New Query Method

1. Add the method to the appropriate query handle in `query.rs`
2. Use `neighbors_by_edge()` with an edge-type predicate for traversal
3. Add a test in `tests/siege_of_silica.rs` or a new test file

## Conventions

- **No `unwrap()` in library code** — return `Result` or `Option`
- **Collect all errors** — validation never short-circuits on first error
- **Edge-type filtering** — use `neighbors_by_edge()`, not `neighbors_directed()` (which ignores edge types)
- **Inclusive TimeSpan, exclusive Allen Interval** — `to_interval()` handles the conversion
- **Test data is real lore** — the Siege of Silica cluster is based on saltglass-steppe world-building

## Test Organization

| File | Purpose |
|------|---------|
| `siege_of_silica.rs` | Happy-path integration tests |
| `validation_errors.rs` | Negative tests — validation catches errors |
| `can_add_event.rs` | Insertion verification + custom config |
| `boundary_conditions.rs` | Edge cases at temporal/state boundaries |
| `subjective_fragments.rs` | Account queries and fidelity |

When adding a feature, write both positive tests (it works) and negative tests (it catches errors). The Allen's interval bug (#1) was invisible until negative tests were added.

## Commit Style

Small, focused commits. Each commit should:
- Do one thing
- Have a descriptive message (imperative mood)
- Reference an issue number if applicable (e.g., `(#8)`)
- Pass clippy + tests

Push frequently. Work close to trunk.
