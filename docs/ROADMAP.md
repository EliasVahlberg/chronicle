# chronicle-graph — Roadmap

> Phased development plan. Each phase produces something usable.
>
> Companion to [SCOPE.md](SCOPE.md) and [PROPOSAL.md](PROPOSAL.md).
>
> **Crate name:** `chronicle-graph` on crates.io, `chronicle` for imports.

## Phase 1 — Graph Model + Load + Validate ✅

**Completed:** `fcf2856`

- Core types: Actor, Place, Event, Concept, Account (TimePeriod deferred)
- RON schema with `{entity_id}` reference syntax in narrative text
- Event metadata: participants with roles and sentiment, location, time_span, caused_by, state_changes
- Directory-based loading: type inferred from subdirectory name
- petgraph `StableGraph` with `Entity`/`Relationship` enums, `HashMap<EntityId, NodeIndex>` index
- 4 validation passes: referential integrity, temporal consistency (Allen's intervals), state tracking, orphan detection
- `ValidationReport` with typed errors and warnings
- Siege of Silica test cluster (18 entities, 5 RON files)

## Phase 2 — Query API ✅

**Completed:** `fcf2856`

- Query handles: `graph.actor()`, `graph.event()`, `graph.place()`, `graph.concept()`
- Method chains: `.interactions()`, `.events_during()`, `.events_at()`, `.causal_chain()`, `.consequences()`, `.participants()`, `.participants_by_role()`
- `InteractionResult` with `.people()` and `.factions()` filters
- `status_at(year)` — reconstructs from Active by scanning state changes
- Text retrieval: `.mentions(entity_id)`, `.accounts_of(event_id)`, `.accounts_by(source_id)`

## Phase 3 — Insertion Verification ✅

**Completed:** `2248116`

- `can_add_event(&Event) -> Result<(), Vec<ValidationError>>` — pure check, no mutation
- `ValidationConfig` with configurable `terminal_statuses` (default: Dead, Destroyed, Dissolved)
- `Chronicle::from_directory_with_config(path, config)` for custom policies
- Actionable error messages with full context (entity IDs, event names, years, specific conflict)
- Negative test data: dangling refs, temporal violations, state violations, orphans, duplicate IDs

## Phase 4 — Subjective Fragments ✅

**Completed:** `525e9e7`

- Accounts as graph nodes with `AuthoredBy`, `AccountOf`, and `Mentions` edges
- `accounts_by(source_id)` — all accounts authored by a given source
- Fidelity filtering by caller (Canonical, Partial, Distorted, Biased, Fabricated, Corrupted)
- Divergence comparison via `parse_references()` — compare entity mentions between accounts
- The Account layer is the subjective fragment; the rest of the graph is the objective truth

## Phase 5 — Saltglass-Steppe Integration

**Status:** Open

- Establish canonical timeline (integer years for all known events)
- Build entity ID registry (canonical IDs for all actors, places, factions)
- Convert one lore cluster from markdown to RON (Schism Wars is the natural starting point)
- Decide game integration pattern (direct dependency vs export)
- Wire one game system to chronicle queries
- CI validation step

**Done when**: At least one game system queries the graph at runtime instead of hardcoded data.

---

## Future: Procedural Graph Generation (separate project)

Not part of this crate. Noted here because it depends on the graph model being stable.

A future project that procedurally generates the **skeleton** of a narrative graph — the metadata structure without the prose. Events, participants, roles, causal chains, temporal ordering, sentiment tags, state changes. The graph skeleton is the outline; prose generation becomes a constrained fill-in task rather than freeform generation.

This is a much more tractable problem than "generate coherent history from scratch" because:
- The graph's validation rules constrain what can be generated (no impossible events)
- Sentiment tags guide tone without requiring understanding of tone
- Causal chains provide narrative structure without requiring plot understanding
- The existing crate's `can_add()` serves as the generator's constraint checker

This would be relevant when saltglass-steppe reaches Tier 4 roadmap items (procedural quests, procedural lore). The graph model needs to be stable and battle-tested on hand-authored content first.

---

## Research Paper

Would naturally emerge after Phase 3. The novel contribution: adapting CIDOC CRM's event-centric model and Allen's Interval Algebra for interactive fiction, with a concrete implementation that supports both authoring-time validation and runtime queries.

Positioning: existing digital humanities tools (EventKG, prosopography frameworks) target researchers analyzing real history. This adapts those models for authored fictional worlds with tighter integration requirements — the graph isn't just for analysis, it's a runtime dependency that game systems query.

The procedural generation future (graph skeleton generation with constraint verification) would be a second paper if it produces interesting results.
