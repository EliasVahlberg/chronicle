# Narrative Knowledge Graphs — Dependency Research

> Date: 2026-04-06
> Status: Complete — ready to create repo

## 1. Existing Rust Crates Survey

**No existing crate does what we need.** The space is open.

| Crate | What It Does | Why Not |
|-------|-------------|---------|
| `narrative-engine` 0.1.0 | Procedural text generation (grammars + Markov chains) | Generates prose, doesn't store/query structured world data |
| `multilinear` 0.6.0 | Petri-net-inspired state machines for interactive fiction | Models story branching/state, not world-building knowledge graphs |
| `ttgraph` 0.5.0 | Typed transactional graph with heterogeneous nodes via derive macros | Interesting design but heavy deps (uuid, indexmap, serde_json), 50% documented, designed for compiler IR |
| `grdf` / `rdftk_core` | RDF triple stores | Too low-level, semantic-web focused, wrong abstraction for narrative |
| `graphica` | Multi-edge graphs with mixed directed/undirected | Too generic, no domain-specific features |

## 2. petgraph Evaluation

**Verdict: Use it.** petgraph gives us graph structure + traversal algorithms for free. We build the typed query API and validation on top.

- **Version**: 0.8.3 (mature, actively maintained)
- **Documentation**: 79%
- **License**: MIT/Apache-2.0
- **Key type**: `StableGraph<N, E>` — keeps indices stable across removals (important for us since entity IDs map to node indices)
- **Node/edge data**: Arbitrary types. We'd use `enum Entity { Actor(..), Event(..), Place(..), Concept(..) }` for nodes and `enum Relationship { ParticipatedIn(..), OccurredAt(..), CausedBy(..) }` for edges
- **Built-in algorithms**: BFS, DFS, Dijkstra, topological sort, connected components — useful for causal chain traversal and subgraph extraction
- **Serde support**: Optional `serde-1` feature for graph serialization
- **Rayon support**: Optional `rayon` feature for parallel iteration
- **DOT export**: Built-in Graphviz output for visualization

**Main friction**: petgraph is homogeneous per graph (one N type, one E type). We handle heterogeneous nodes/edges via enums. This is the standard Rust pattern and works fine — the typed query API wraps the enum matching.

**Alternative considered**: Custom adjacency list. Simpler but we'd lose all the built-in algorithms. Not worth it unless petgraph proves too heavy (unlikely — it's a lightweight crate).

## 3. Allen's Interval Algebra

**Verdict: Use `allen-intervals` crate.** Exactly what we need, no reason to reimplement.

- **Crate**: `allen-intervals` 0.1.0 (by regexident)
- **Published**: May 2025
- **Documentation**: 100%
- **License**: MPL-2.0
- **Dependencies**: Only optional `thiserror`

Implements all 13 Allen relations as traits:
- `Precedes` / `is_preceded_by`
- `Meets` / `is_met_by`
- `Overlaps` / `is_overlapped_by`
- `Starts` / `is_started_by`
- `During` / `contains`
- `Finishes` / `is_finished_by`
- `Equals`

Supports both discrete (integer) and continuous (float) time domains. Our timeline uses integer years, so discrete mode works directly:

```rust
use allen_intervals::{Interval, NonEmpty, Precedes};

let birth: NonEmpty<_> = Interval { start: 10, end: 10 }.try_into().unwrap();
let siege: NonEmpty<_> = Interval { start: 20, end: 20 }.try_into().unwrap();

assert!(birth.precedes(&siege)); // Kaine was born before the siege — valid
```

## 4. RON Ergonomics (Prototype)

**Verdict: RON works well for hand-authoring.** See `prototype_siege_of_silica.ron`.

Prototyped a real saltglass-steppe event cluster: 5 factions, 5 characters, 2 places, 5 events (causal chain), 1 subjective account.

### What works
- Nested structs for participants with role + sentiment are readable
- Entity references as string IDs work in both structured fields and narrative text (`{entity_id}`)
- Separate files per category (actors/, places/, events/, accounts/) feels natural
- Status tracking (`status` + `status_since_event`) is ergonomic
- Sentiment per participant captures the subjective dimension well
- Causal chains via `caused_by: ["event_id"]` are clear

### What's slightly awkward
- Optional fields (`born: None, died: None`) are verbose but acceptable
- `[$time_ref]` syntax in prose wasn't needed — `time_span` on events is sufficient for now
- RON doesn't validate references at parse time — that's our crate's job (expected)
- No syntax highlighting for `{entity_id}` references in editors (cosmetic)

### Decision
Start with RON. If authoring friction becomes significant after converting more lore, revisit the custom format question. The prototype suggests RON is adequate.

## 5. Proposed Dependency Stack

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
ron = "0.8"
petgraph = { version = "0.8", features = ["serde-1", "stable_graph"] }
allen-intervals = "0.1"
thiserror = "2"
```

Minimal. No async, no database, no network, no heavy deps.

## 6. Open Questions Resolved

| Question | Answer |
|----------|--------|
| Custom graph vs petgraph? | petgraph — get algorithms for free, wrap with typed API |
| Reimplement Allen's algebra? | No — `allen-intervals` crate is perfect |
| RON vs JSON vs custom format? | RON to start, revisit if painful |
| Existing narrative graph crate? | None exists in Rust. Space is open. |

## 7. Ready to Create Repo

All dependencies evaluated. RON schema prototyped with real lore. No blockers.

Next step: `cargo init` the crate, define the core types, implement load + validate for the Siege of Silica cluster.
