# Narrative Knowledge Graphs — Roadmap

> Phased development plan. Each phase produces something usable.
>
> Companion to [SCOPE.md](SCOPE.md) and [PROPOSAL.md](PROPOSAL.md).

## Phase 1 — Graph Model + Load + Validate

**Goal**: The crate exists. It loads RON files, builds a typed graph, and catches inconsistencies.

This is the foundation. Getting the data model right matters more than anything else. Test with a small cluster of real saltglass-steppe lore (e.g., the Siege of Silica and its surrounding events/actors/places).

- Define core types: Actor, Place, Event, Concept, TimePeriod
- Define RON schema with `{entity_id}` and `[$time_ref]` reference syntax in narrative text
- Event metadata: participants with roles and sentiment, location, time_span, caused_by, state_changes
- Sentiment tags per participant per event (Triumphant, Devastating, Sorrowful, Neutral, Resentful, etc.)
- Load from directory, resolve all references, build in-memory graph
- Referential validation: no dangling refs, no orphans
- Temporal validation: Allen's Interval Algebra — lifespan checks, causal ordering, dead actors don't participate
- Output: `ValidationReport` with typed errors and warnings
- ~~Decide: petgraph vs custom adjacency structure~~ → Decided: petgraph `StableGraph`

**Done when**: A real lore cluster loads, validates, and the types feel right for authoring.

## Phase 2 — Query API

**Goal**: The crate is useful. Typed traversals answer real questions about the world.

- Query handles: `graph.actor()`, `graph.event()`, `graph.place()`, `graph.concept()`
- Method chains: `.interactions()`, `.events_during()`, `.causal_chain()`, `.participants()`, `.participants_by_role()`
- Sentiment queries: `.events_with_sentiment(Devastating)`, participant-level sentiment filtering
- Subgraph extraction with depth and type filters
- Text retrieval: `.mentions(entity_id)`, `.accounts_of(event_id)`
- Iteration/collection: query results as iterators over typed handles

**Done when**: A real saltglass-steppe question ("what factions were involved in events at Silica Citadel between year 800 and 900?") is answerable as a method chain.

## Phase 3 — Insertion Verification

**Goal**: The crate prevents problems. New content is validated before it enters the graph.

- `can_add(&event)` — pre-insertion consistency check against temporal, spatial, and state constraints
- State tracking: events produce state changes (death, destruction, dissolution), future operations respect them
- Incremental validation: don't re-validate the whole graph on every insert
- Clear error messages: "Cannot add: actor 'kael' has status Deceased as of event 'battle_of_x' (year 835)"

**Done when**: Attempting to add an impossible event returns a specific, actionable rejection.

## Phase 4 — Subjective Fragments

**Goal**: The crate models unreliable narrators. Different sources have different views of the same events.

- Fidelity ratings on accounts: Canonical, Partial, Distorted, Biased, Fabricated, Corrupted
- Objective graph as union of Canonical-fidelity sources
- Subjective fragments attributed to specific sources (NPCs, books, archive-drones)
- Divergence queries: "where does this account differ from ground truth?"
- Per-source sentiment: the same event has different sentiment depending on who's telling it
- Archive-drones as high-fidelity sources, NPCs as partial/biased sources

**Done when**: Two conflicting accounts of the same event coexist in the graph with queryable divergence.

## Phase 5 — Saltglass-Steppe Integration

**Goal**: The crate is wired into the game. Lore is structured, validated, and queryable at runtime.

- Convert a meaningful chunk of lore from markdown to RON (start with a single event cluster, not everything)
- Wire into game systems: spawn queries, quest triggers, NPC dialogue source selection
- CI validation step: `cargo test` fails if lore graph has consistency errors
- Deprecate corresponding markdown lore files as they're converted

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
