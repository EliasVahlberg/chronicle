---
name: domain-knowledge
description: Chronicle's problem domain, design decisions, and theoretical foundations. Use when making architectural decisions, evaluating approaches, or understanding why the system works the way it does.
---

# Chronicle — Domain Knowledge

## What Chronicle Is

A Rust crate that loads authored narrative content into a typed, event-centric knowledge graph, validates it for consistency, and provides a query interface for traversal and subgraph extraction. The primary value is **verification** — scoring or rejecting proposed records against the existing graph based on temporal, spatial, and state constraints.

## The Problem It Solves

World-building for narrative-heavy games accumulates as disconnected documents that drift into inconsistency. Characters contradict themselves across files, timelines break, AI-generated content introduces details that conflict with established canon. At scale (40+ documents), no human can hold the world in their head. Chronicle enforces consistency structurally.

## Event-Centric Model

Adapted from CIDOC CRM's core principle (not the full ISO standard): **events are the connective tissue between actors, places, and time**. Instead of linking a character directly to a location, every connection passes through an event node that carries metadata (time, participants with roles and sentiment, location, causal links, state changes).

Entity types (will grow as the model meets real content):
- **Actor**: Characters, factions, deities, organizations, creatures
- **Place**: Settlements, regions, dungeons, landmarks, ruins
- **Event**: Battles, sieges, discoveries, political events, migrations, deaths
- **Concept**: Religions, technologies, artifacts, laws
- **Account**: Subjective narrative text with `{entity_id}` references and fidelity ratings

## Temporal Verification (Allen's Interval Algebra)

Events have time spans. Verification uses Allen's 13 interval relations to check:
- Actor participation falls within their lifespan
- Causal chains are temporally ordered (cause precedes effect)
- State changes are respected (dead actors don't participate in later events)
- Destroyed places can't host events without ruin context

## Objective vs. Subjective Graphs

- **Objective graph**: Ground truth — union of all Canonical-fidelity content
- **Subjective fragments**: What a specific NPC/book/faction believes happened. Typed fidelity: Canonical, Partial, Distorted, Biased, Fabricated, Corrupted
- This maps to gameplay: archive-drones have high fidelity, NPCs have partial/biased fragments, the player's understanding grows as they encounter more sources

## Key Design Decisions

- **RON format** for authored content (prototyped, ergonomic enough)
- **petgraph StableGraph** for the in-memory graph (stable indices, built-in algorithms)
- **allen-intervals crate** for temporal verification (all 13 relations, discrete integer years)
- **Typed query API** — method chains, not natural language
- **Entity references as string IDs** in both structured fields and narrative prose (`{entity_id}`)
- **Sentiment per participant per event** — not per event globally

## First Consumer

saltglass-steppe — a deterministic TUI roguelike RPG with ~45 lore documents covering world history, factions, characters, locations, creatures, materials, and psychic systems.
