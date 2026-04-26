# Narrative Knowledge Graphs — Project Proposal

> Structured world-building as queryable, verifiable entity-relation graphs.
>
> Status: Proposal
> Date: 2026-04-06

## The Problem

World-building for narrative-heavy games accumulates as disconnected markdown files — histories, bestiaries, character profiles, faction descriptions, location guides. Over time:

1. **Inconsistency**: Information drifts. A character's faction allegiance in one document contradicts another. A historical event's date shifts between files. AI-generated content introduces plausible-sounding details that conflict with established canon.
2. **Orphaned content**: Lore fragments exist without connection to anything else. They can't be validated, queried, or tied to gameplay systems.
3. **Stale context**: AI coding agents read lore documents at face value. When documents contradict each other, the agent picks whichever it read last. The result is implementations that reference entities, events, or relationships that don't exist or have changed.
4. **Scaling failure**: At small scale, a human can hold the world in their head. At 40+ lore documents and growing, nobody can. The inconsistencies compound faster than they can be caught.
5. **No retrieval structure**: Finding "who has this character interacted with?" requires reading every document. There's no way to traverse relationships, and no way to answer graph queries over the world state.

This is observable in saltglass-steppe today: ~45 narrative documents, growing AI-assisted content, and visible inconsistencies where the world-building stops making sense.

## The Proposal

Replace disconnected prose documents with structured entity-relation graphs. Every piece of world-building — character, faction, location, event, item, concept — is a typed node with explicit, validated relationships to other nodes.

A Rust crate that:
- Defines a typed, event-centric graph model for narrative world-building
- Parses authored content into that graph
- Validates consistency (no dangling references, no contradictory relationships, temporal/spatial plausibility)
- Provides a query interface for graph traversal and subgraph extraction
- Can be integrated into a game's build or runtime to tie lore directly to gameplay entities

### What It Is Not

- Not a game engine or narrative runtime
- Not a procedural text generator
- Not a wiki or documentation tool
- Not an AI/LLM system — it's the structured ground truth that AI-generated content gets validated against

## Theoretical Foundation: Event-Centric Knowledge Graphs

The approach draws on how historians and digital humanities researchers model real historical accounts. See [INITIAL_RESEARCH.md](INITIAL_RESEARCH.md) for full sources.

### CIDOC CRM — Influence, Not Implementation

CIDOC CRM (ISO 21127:2023) is the international standard ontology for cultural heritage documentation. It defines ~90 entity classes, ~160 properties, and a formal RDF/OWL representation designed for museum interoperability. We don't implement CIDOC CRM — it's far too large and domain-specific for our needs.

What we adopt is its core architectural principle: **events are the connective tissue between actors, places, and time**. Instead of linking a character directly to a location, every connection passes through an event node. In CIDOC CRM terms, this is the pattern where E21 Person connects to E53 Place through E5 Event via P11 (had participant) and P7 (took place at).

Our model starts with a small set of entity types (Actor, Place, Event, Concept) but is designed to grow as real content demands it. CIDOC CRM serves as a reference we can continue borrowing from — additional entity types, relationship patterns, or temporal modeling concepts can be adopted incrementally as the model matures.

What we don't take from CIDOC CRM: the deep class hierarchy, the RDF/OWL formalism, the museum-specific domain classes, or the interoperability goal. Our model is optimized for authoring-time verification and runtime queries over fictional worlds, not for cross-institutional data exchange.

### Allen's Interval Algebra — Temporal Verification

Temporal consistency uses Allen's Interval Algebra, which defines 13 possible relationships between time intervals (before, after, during, overlaps, meets, etc.). This enables:

- **Pre-condition checking**: Before adding "Kael participated in the Siege of Silica (847)", verify that Kael's birth date precedes 847 and death date follows it.
- **Causal ordering**: If Event A caused Event B, A's time span must precede or overlap B's.
- **State tracking**: Events produce world-state changes. If a character dies in Event A, they cannot participate in Event B afterward.

### Spatio-Temporal Plausibility

Beyond temporal ordering, the graph can verify spatial plausibility: could an actor physically travel between two event locations in the time between them? This uses the world's geography (distances, terrain, travel speeds) to validate whether a recorded sequence of events is possible.

### Objective vs. Subjective Graphs

A distinction from game narrative research:
- **Objective graph**: The ground truth of what actually happened in the world's history. The canonical record.
- **Subjective fragments**: Small sub-graphs representing what a specific NPC, book, or faction *believes* happened. These can be intentionally biased, incomplete, or wrong — but their relationship to the objective graph is explicit and queryable.

Each subjective fragment has a **fidelity** relationship to the objective graph — how much it overlaps with ground truth, and where it diverges. Divergences are typed: omission (they don't know), distortion (they remember wrong), fabrication (they're lying), or corruption (the record was damaged).

This maps directly to gameplay in saltglass-steppe:

- **NPCs** give the player their subjective fragment. A faction propagandist's fragment is deliberately distorted. A paranoid hermit's is incomplete. Two NPCs tell conflicting stories about the Schism Wars — the player learns to cross-reference.
- **Archive-drones** (already in the lore as part of the Archive Consciousness) have near-perfect recollection. Their fragments have high fidelity to the objective graph — they're the in-world justification for reliable sources. Finding an archive-drone becomes mechanically meaningful: it's how you get ground truth.
- **Degraded records** introduce uncertainty even in "reliable" sources. A storm-damaged archive-drone's fragment has corruption markers on specific nodes. The player can see *that* information is missing, even if they can't see *what* it was.

The objective/subjective split isn't just a data modeling convenience — it's a game mechanic. The player's understanding of the world is itself a subjective fragment that grows and self-corrects as they encounter more sources. Trust and reliability become queryable properties, not vibes.

## Core Capabilities

The central value proposition is **verification** — the ability to score or reject a proposed record against the existing graph based on temporal, spatial, and state constraints. Storage and querying support this goal.

### 1. Consistency Verification (primary)

The defining capability. Before inserting a new record, the graph validates:

- **Temporal**: Does this event's time span conflict with its participants' lifespans or other commitments? (Allen's Interval Algebra)
- **Spatial**: Could this actor have traveled from their last known location to this event's location in the elapsed time?
- **Causal**: Does this event's cause actually precede it?
- **State**: Are all participants alive/extant at the time of the event? Has a referenced place been destroyed?
- **Referential**: Do all referenced nodes exist? No dangling edges.

The query: "Can we add a record that this person was at {place} at {time} with {person}?" becomes a formal validation that returns a verdict with specific reasons for acceptance or rejection.

### 2. Graph-Based Retrieval and Search

The primary interface is graph traversal queries:

- "What people has this character interacted with in recorded history?" → traverse all events involving the character, collect co-participants
- "What events occurred at this location?" → filter events by place
- "What is the causal chain leading to this event?" → follow caused_by edges backward
- "What factions were active during this time period?" → temporal range query over faction participation in events
- Subgraph extraction: "Give me everything related to the Schism Wars" → BFS/DFS from the event node, collecting all connected entities within N hops

### 3. State Tracking Over Time

Events produce state changes on nodes:
- A character dies → status changes to Deceased, future participation blocked
- A city is destroyed → status changes to Destroyed, future events there require Ruins context
- A faction splits → two new faction nodes, membership edges redistributed

The graph maintains a temporal state for every node, queryable at any point in the timeline.

## First Consumer: saltglass-steppe

Saltglass-steppe has ~45 lore documents covering:
- World history (Schism Wars, Day of Broken Mirrors, Heliograph Expedition)
- Factions (Mirror Order, Salt Merchants, Void Seekers, etc.)
- Characters (Dramatis Personae, NPCs, quest-givers)
- Locations (Atlas of Glass, Regional Gazetteer)
- Creatures (Quantum Bestiary)
- Materials and physics (Lithopedia of Glass, Photonic Ecology)
- Psychic systems (Psychic Codex, Refraction Heresy)

Integration points:
1. **Spawn system**: "What creatures are native to this biome?" → graph query
2. **Quest system**: "What events involved this faction?" → graph traversal
3. **NPC dialogue**: Draw from the NPC's subjective fragment of the graph
4. **Procedural lore generation**: Generate new events that pass the graph's consistency checks
5. **Content validation**: CI step that verifies all lore content parses into a consistent graph

## Open Design Questions

### File Format

Options:
1. **Existing format (JSON/RON/TOML)**: Zero parser work. Verbose for hand-authoring graph data.
2. **Annotated markdown**: Human-readable prose with structured annotations that parse into graph nodes.
3. **Custom DSL**: A domain-specific format for authoring narrative graphs ergonomically. More work (PEG grammar via `pest` or `winnow`), but potentially a genuine contribution if the domain warrants it.

Decision deferred until prototyping. Start with RON/JSON, feel the pain points, decide if a custom format earns its complexity.

**Update (2026-04-06):** RON selected after prototyping the Siege of Silica cluster. See [RESEARCH.md](RESEARCH.md) §4.

### Graph Model Details

- Fixed schema (predefined node/edge types) vs. flexible schema (user-defined types)?
- How to represent "this was true during this period but not after"? (Allen's intervals are the likely answer)
- Confidence/canonicity: how to mark speculative vs. established lore?
- Hierarchical grouping: can a subgraph represent "everything in Act II"?
- How much of CIDOC CRM to adopt vs. simplify for game use?

### Query Interface

- Traversal queries (reachable from node X within N hops)
- Filtered subgraph extraction (all nodes of type Character with edge to Faction Y)
- Temporal range queries (all events between Year 800 and Year 900)
- Consistency queries (find contradictions, orphans, temporal/spatial violations)
- "Can I add this?" validation queries
- Export to other formats (markdown summary, JSON for game data files)

## Research Angle

The research direction will emerge from the implementation work, but the likely contribution centers on **verification of narrative records against an event-centric knowledge graph**.

The core question: given a graph of established world-building facts (actors, events, places, temporal relationships, causal chains, state changes), can we formally evaluate whether a proposed new record is consistent? And can we give a specific, actionable verdict — not just "invalid" but "invalid because actor X has status Dead as of event Y (year Z)"?

This draws on:
- **CIDOC CRM's event-centric principle** — events as the connective tissue, adapted and simplified for fictional worlds. Additional CIDOC CRM concepts can be adopted incrementally as the model matures.
- **Allen's Interval Algebra** — temporal constraint checking, applied as a prescriptive tool (what *can* happen) rather than a descriptive one (what *did* happen).
- **The objective/subjective graph split** — modeling epistemic uncertainty in narrative as a queryable data structure.

Positioning: existing digital humanities tools (EventKG, prosopography frameworks) analyze real historical data after the fact. Chronicle inverts this — it validates fictional records *before* they enter the canon. The same formal methods, different direction of application.

## Relationship to Other Projects

| Project | Relationship |
|---------|-------------|
| saltglass-steppe | First consumer. Lore documents become graph content. Game systems query the graph. |
| terrain-forge | None directly. Both are published crates consumed by saltglass-steppe. |
| ogun | None directly. |
| fishy | Methodological parallel — both are "structured analysis of unstructured content" tools. |

## Timeline Considerations

This is not urgent. The current priority for saltglass-steppe is feature development (Tier 2 roadmap items: mob/loot overhaul, adaptations rework, storm system). The lore inconsistency problem is real but manageable at current scale.

Suggested trigger: when the next round of content expansion (Tier 4: main questline, procedural quests, procedural lore) makes the inconsistency problem acute enough to block progress.

In the meantime: the proposal exists, the problem is documented, the theoretical foundation is identified, and the approach is scoped.
