# Workflows

<!-- Generated: 2026-05-01 | tags: workflows, processes, pipelines -->

## Loading a World

```mermaid
sequenceDiagram
    participant U as User
    participant C as Chronicle
    participant FS as Filesystem
    participant G as StableGraph

    U->>C: from_directory("world/")
    C->>FS: Walk actors/, places/, events/, concepts/, accounts/
    FS-->>C: RON file contents
    C->>C: Deserialize Vec<T> per subdirectory
    C->>C: Check for duplicate IDs
    C->>G: Add nodes (Entity enum)
    C->>C: Collect edge tuples (source_nx, target_id, Relationship)
    C->>G: Resolve target IDs → NodeIndex, add edges
    C-->>U: Ok(Chronicle) or Err(ChronicleError)
```

Type inference: subdirectory name determines the deserialization target. `actors/*.ron` → `Vec<Actor>`, etc. Files within a subdirectory can have any name.

## Validating a Graph

```mermaid
flowchart TD
    V[chronicle.validate] --> REF[Referential Pass]
    V --> TEMP[Temporal Pass]
    V --> STATE[State Pass]
    V --> ORPHAN[Orphan Pass]

    REF -->|"All ID refs resolve?"| REPORT[ValidationReport]
    TEMP -->|"Lifespans contain events? Causes precede effects?"| REPORT
    STATE -->|"Terminal entities don't participate later?"| REPORT
    ORPHAN -->|"Zero in+out degree?"| REPORT

    REPORT --> ERRORS[errors: Vec of ValidationError]
    REPORT --> WARNINGS[warnings: Vec of ValidationWarning]
```

All passes run independently. Errors are collected, not short-circuited.

## Checking a Proposed Event

```mermaid
sequenceDiagram
    participant U as User
    participant C as Chronicle

    U->>C: can_add_event(&proposed_event)
    C->>C: Check: ID not already in graph?
    C->>C: Check: all participants exist?
    C->>C: Check: location exists?
    C->>C: Check: caused_by events exist?
    C->>C: Check: state_change targets exist?
    C->>C: Check: participant lifespans contain event?
    C->>C: Check: causes precede effect?
    C->>C: Check: no participant/location in terminal state?
    alt All checks pass
        C-->>U: Ok(())
    else Violations found
        C-->>U: Err(Vec of ValidationError)
    end
```

Pure check — does not mutate the graph.

## Querying the Graph

```mermaid
flowchart LR
    C[chronicle] -->|".actor(id)"| AQ[ActorQuery]
    C -->|".event(id)"| EQ[EventQuery]
    C -->|".place(id)"| PQ[PlaceQuery]
    C -->|".concept(id)"| CQ[ConceptQuery]

    AQ -->|".events()"| EVENTS[Vec of Event]
    AQ -->|".interactions()"| IR[InteractionResult]
    AQ -->|".status_at(year)"| STATUS[Status]

    EQ -->|".causal_chain()"| CHAIN[Vec of Event via BFS]
    EQ -->|".consequences()"| CONS[Vec of Event via edges]
    EQ -->|".participants_by_role()"| PARTS[Vec of Participant]

    PQ -->|".actors_present_at(year)"| ACTORS[Vec of Actor]

    C -->|".mentions(id)"| ACCS[Vec of Account]
    C -->|".accounts_of(id)"| ACCS
    C -->|".accounts_by(id)"| ACCS
```

All queries are read-only borrows. Return `Option` for single lookups, `Vec` for collections.

## Authoring Content

```
1. Create RON files in the appropriate subdirectory:
   world/actors/my_characters.ron → Vec<Actor>
   world/events/my_events.ron    → Vec<Event>

2. Use {entity_id} references in account text:
   text: "The {mirror_order} attacked {silica}."

3. Load and validate:
   let graph = Chronicle::from_directory("world/")?;
   let report = graph.validate();

4. Check proposed additions:
   graph.can_add_event(&new_event)?;
```

## Temporal Verification Detail

```mermaid
flowchart TD
    TS["TimeSpan {start: 20, end: 20}"] -->|"to_interval()"| AI["Allen Interval {start: 20, end: 21}"]
    AI -->|"strictly_before(a, b)"| CHECK{"a.precedes(b) || a.meets(b)"}
    CHECK -->|true| BEFORE["a is strictly before b"]
    CHECK -->|false| NOTBEFORE["a overlaps, contains, or follows b"]
```

Key: TimeSpan uses inclusive bounds. Allen's discrete intervals use exclusive end. The conversion adds 1 to the end. `strictly_before` combines `precedes` (gap between intervals) and `meets` (adjacent, no gap) to handle the discrete year boundary correctly.
