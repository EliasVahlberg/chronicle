# Chronicle — Documentation Index

<!-- Generated: 2026-05-01 | Primary context file for AI assistants -->

## How to Use This Index

This file is the entry point for understanding the chronicle codebase. It contains enough metadata about each documentation file that you can determine which file to consult for any given question without reading all of them.

**For AI assistants**: Load this file first. Use the summaries below to decide which detailed file to read. Most questions can be answered by consulting 1–2 files from this index.

## Quick Reference

| Question | Consult |
|----------|---------|
| What types/enums exist? | `data_models.md` — all entity types, enums, relationships |
| What's the public API? | `interfaces.md` — full API surface with signatures |
| How does loading work? | `workflows.md` — loading sequence, type inference |
| How does validation work? | `architecture.md` — validation passes; `workflows.md` — sequence detail |
| How do temporal checks work? | `architecture.md` — Allen's intervals; `workflows.md` — conversion detail |
| What are the dependencies? | `dependencies.md` — each crate's purpose and integration |
| What modules exist? | `components.md` — module map and responsibilities |
| Project metadata? | `codebase_info.md` — version, deps, file structure |
| Known issues? | `review_notes.md` — inconsistencies, gaps, recommendations |

## Documentation Files

### codebase_info.md
**Purpose**: Project metadata, build configuration, file structure.
**Contains**: Package info, dependency table, source tree, test tree.
**Consult when**: You need basic project facts or want to understand file organization.

### architecture.md
**Purpose**: System design and key decisions.
**Contains**: Event-centric model explanation, pipeline architecture, graph representation (petgraph StableGraph), validation architecture (4 independent passes), design decision table.
**Consult when**: You need to understand *why* the system works the way it does, or how components relate to each other.

### components.md
**Purpose**: Module responsibilities and relationships.
**Contains**: Module dependency diagram, per-module descriptions (model.rs, graph.rs, query.rs, validation.rs, error.rs), query handle table, edge traversal pattern.
**Consult when**: You need to find which file handles a specific feature, or understand what a module does.

### interfaces.md
**Purpose**: Complete public API surface.
**Contains**: All public method signatures for Chronicle, query handles, validation types, configuration, RON format conventions, error handling patterns.
**Consult when**: You need to write code that uses chronicle, or understand the exact API shape.

### data_models.md
**Purpose**: All types, enums, and their fields.
**Contains**: Entity class diagram, relationship table (10 edge types), sub-type enums, behavioral enums, supporting types, serde conventions.
**Consult when**: You need to understand the data model, add a new entity type, or modify existing types.

### workflows.md
**Purpose**: Step-by-step processes and data flows.
**Contains**: Loading sequence, validation flow, can_add_event flow, query flow, authoring workflow, temporal verification detail (TimeSpan → Allen Interval conversion).
**Consult when**: You need to trace how a specific operation flows through the system.

### dependencies.md
**Purpose**: External crate usage and integration details.
**Contains**: Dependency graph, per-crate integration details (petgraph, allen-intervals, serde+ron, thiserror), key gotchas (Allen's discrete exclusive end bounds).
**Consult when**: You need to understand why a crate is used or how it integrates.

### review_notes.md
**Purpose**: Documentation quality assessment.
**Contains**: Consistency check results, completeness gaps, recommendations.
**Consult when**: You want to know what's missing or needs improvement.
