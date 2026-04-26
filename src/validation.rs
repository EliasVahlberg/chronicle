//! Validation passes: referential integrity, temporal consistency, state tracking, orphan detection.

use std::fmt;

use allen_intervals::{Interval, Meets, NonEmpty, Precedes};
use petgraph::Direction;

use crate::graph::{Chronicle, parse_references};
use crate::model::*;

// ── Report types ───────────────────────────────────────────

#[derive(Debug, Default)]
pub struct ValidationReport {
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

impl ValidationReport {
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
}

#[derive(Debug)]
pub enum ValidationError {
    DanglingReference {
        source_id: String,
        target_id: String,
        context: String,
    },
    TemporalViolation {
        entity_id: String,
        event_id: String,
        description: String,
    },
    StateViolation {
        entity_id: String,
        event_id: String,
        description: String,
    },
}

#[derive(Debug)]
pub enum ValidationWarning {
    OrphanEntity { entity_id: String },
    TemporalAmbiguity { description: String },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DanglingReference { source_id, target_id, context } => {
                write!(f, "dangling reference: {source_id} -> {target_id} ({context})")
            }
            Self::TemporalViolation { entity_id, event_id, description } => {
                write!(f, "temporal violation: {entity_id} in {event_id}: {description}")
            }
            Self::StateViolation { entity_id, event_id, description } => {
                write!(f, "state violation: {entity_id} in {event_id}: {description}")
            }
        }
    }
}

impl fmt::Display for ValidationWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OrphanEntity { entity_id } => {
                write!(f, "orphan entity: {entity_id} has no references in or out")
            }
            Self::TemporalAmbiguity { description } => {
                write!(f, "temporal ambiguity: {description}")
            }
        }
    }
}

// ── Validation entry point ─────────────────────────────────

pub fn validate(chronicle: &Chronicle) -> ValidationReport {
    let mut report = ValidationReport::default();
    validate_referential(chronicle, &mut report);
    validate_temporal(chronicle, &mut report);
    validate_state(chronicle, &chronicle.config, &mut report);
    validate_orphans(chronicle, &mut report);
    report
}

// ── Referential integrity ──────────────────────────────────

fn validate_referential(chronicle: &Chronicle, report: &mut ValidationReport) {
    for nx in chronicle.graph.node_indices() {
        let entity = &chronicle.graph[nx];
        match entity {
            Entity::Actor(actor) => {
                for aff in &actor.affiliations {
                    check_ref(chronicle, &actor.id, aff, "affiliation", report);
                }
                if let Some(ev) = &actor.status_since_event {
                    check_ref(chronicle, &actor.id, ev, "status_since_event", report);
                }
            }
            Entity::Place(place) => {
                if let Some(r) = &place.region {
                    check_ref(chronicle, &place.id, r, "region", report);
                }
                if let Some(ev) = &place.status_since_event {
                    check_ref(chronicle, &place.id, ev, "status_since_event", report);
                }
            }
            Entity::Event(event) => {
                if let Some(loc) = &event.location {
                    check_ref(chronicle, &event.id, loc, "location", report);
                }
                for cause in &event.caused_by {
                    check_ref(chronicle, &event.id, cause, "caused_by", report);
                }
                for p in &event.participants {
                    check_ref(chronicle, &event.id, &p.actor, "participant", report);
                }
                for sc in &event.state_changes {
                    check_ref(chronicle, &event.id, &sc.entity, "state_change target", report);
                }
            }
            Entity::Concept(concept) => {
                if let Some(origin) = &concept.origin_event {
                    check_ref(chronicle, &concept.id, origin, "origin_event", report);
                }
            }
            Entity::Account(account) => {
                check_ref(chronicle, &account.id, &account.source, "source", report);
                for ev in &account.event_refs {
                    check_ref(chronicle, &account.id, ev, "event_ref", report);
                }
                for mention in parse_references(&account.text) {
                    check_ref(chronicle, &account.id, &mention, "text mention", report);
                }
            }
        }
    }
}

fn check_ref(
    chronicle: &Chronicle,
    source_id: &str,
    target_id: &str,
    context: &str,
    report: &mut ValidationReport,
) {
    if !chronicle.index.contains_key(target_id) {
        report.errors.push(ValidationError::DanglingReference {
            source_id: source_id.to_owned(),
            target_id: target_id.to_owned(),
            context: context.to_owned(),
        });
    }
}

// ── Temporal consistency ───────────────────────────────────

fn to_interval(ts: &TimeSpan) -> Option<NonEmpty<Interval<i32>>> {
    // TimeSpan uses inclusive bounds; Allen's discrete intervals use exclusive end.
    Interval { start: ts.start, end: ts.end + 1 }.try_into().ok()
}

/// True if interval `a` ends strictly before `b` starts (precedes or meets in Allen's algebra).
fn strictly_before(a: &NonEmpty<Interval<i32>>, b: &NonEmpty<Interval<i32>>) -> bool {
    a.precedes(b) || a.meets(b)
}

fn validate_temporal(chronicle: &Chronicle, report: &mut ValidationReport) {
    for nx in chronicle.graph.node_indices() {
        if let Entity::Event(event) = &chronicle.graph[nx] {
            let Some(event_iv) = to_interval(&event.time_span) else { continue };

            // Check participants' lifespans
            for p in &event.participants {
                if let Some(&actor_nx) = chronicle.index.get(&p.actor)
                    && let Entity::Actor(actor) = &chronicle.graph[actor_nx]
                    && let Some(lifespan) = &actor.lifespan
                    && let Some(life_iv) = to_interval(lifespan)
                {
                    if strictly_before(&event_iv, &life_iv) {
                        report.errors.push(ValidationError::TemporalViolation {
                            entity_id: actor.id.clone(),
                            event_id: event.id.clone(),
                            description: format!(
                                "event at {}-{} precedes actor lifespan {}-{}",
                                event.time_span.start, event.time_span.end,
                                lifespan.start, lifespan.end,
                            ),
                        });
                    }
                    if strictly_before(&life_iv, &event_iv) {
                        report.errors.push(ValidationError::TemporalViolation {
                            entity_id: actor.id.clone(),
                            event_id: event.id.clone(),
                            description: format!(
                                "actor lifespan {}-{} ends before event at {}-{}",
                                lifespan.start, lifespan.end,
                                event.time_span.start, event.time_span.end,
                            ),
                        });
                    }
                }
            }

            // Check causal ordering: cause must not be after effect
            for cause_id in &event.caused_by {
                if let Some(&cause_nx) = chronicle.index.get(cause_id)
                    && let Entity::Event(cause_event) = &chronicle.graph[cause_nx]
                    && let Some(cause_iv) = to_interval(&cause_event.time_span)
                    && strictly_before(&event_iv, &cause_iv)
                {
                    report.errors.push(ValidationError::TemporalViolation {
                        entity_id: event.id.clone(),
                        event_id: cause_event.id.clone(),
                        description: format!(
                            "effect {}-{} precedes its cause {}-{}",
                            event.time_span.start, event.time_span.end,
                            cause_event.time_span.start, cause_event.time_span.end,
                        ),
                    });
                    // cause precedes/meets effect = valid; overlap/equal = ambiguous, not error
                }
            }
        }
    }
}

// ── State tracking ─────────────────────────────────────────

fn validate_state(chronicle: &Chronicle, config: &ValidationConfig, report: &mut ValidationReport) {
    let state_changes = collect_state_changes(chronicle);

    for nx in chronicle.graph.node_indices() {
        if let Entity::Event(event) = &chronicle.graph[nx] {
            let Some(event_iv) = to_interval(&event.time_span) else { continue };

            // Check participants aren't in a terminal state from a strictly earlier event
            for p in &event.participants {
                for (entity_id, status, change_time, change_event_id) in &state_changes {
                    if *entity_id == p.actor
                        && config.terminal_statuses.contains(status)
                        && let Some(change_iv) = to_interval(change_time)
                        && strictly_before(&change_iv, &event_iv)
                    {
                        report.errors.push(ValidationError::StateViolation {
                            entity_id: p.actor.clone(),
                            event_id: event.id.clone(),
                            description: format!(
                                "actor '{}' has status {:?} as of '{}' (year {}-{}), \
                                 cannot participate in '{}' (year {}-{})",
                                p.actor, status, change_event_id,
                                change_time.start, change_time.end,
                                event.id, event.time_span.start, event.time_span.end,
                            ),
                        });
                    }
                }
            }

            // Check locations aren't in a terminal state
            if let Some(loc) = &event.location {
                for (entity_id, status, change_time, change_event_id) in &state_changes {
                    if entity_id == loc.as_str()
                        && config.terminal_statuses.contains(status)
                        && let Some(change_iv) = to_interval(change_time)
                        && strictly_before(&change_iv, &event_iv)
                    {
                        report.errors.push(ValidationError::StateViolation {
                            entity_id: loc.clone(),
                            event_id: event.id.clone(),
                            description: format!(
                                "location '{}' has status {:?} as of '{}' (year {}-{}), \
                                 cannot host '{}' (year {}-{})",
                                loc, status, change_event_id,
                                change_time.start, change_time.end,
                                event.id, event.time_span.start, event.time_span.end,
                            ),
                        });
                    }
                }
            }
        }
    }
}

// ── Orphan detection ───────────────────────────────────────

fn validate_orphans(chronicle: &Chronicle, report: &mut ValidationReport) {
    for nx in chronicle.graph.node_indices() {
        let in_deg = chronicle.graph.neighbors_directed(nx, Direction::Incoming).count();
        let out_deg = chronicle.graph.neighbors_directed(nx, Direction::Outgoing).count();
        if in_deg == 0 && out_deg == 0 {
            report.warnings.push(ValidationWarning::OrphanEntity {
                entity_id: chronicle.graph[nx].id().to_owned(),
            });
        }
    }
}

// ── Shared helpers ─────────────────────────────────────────

/// Collect all status changes from events: (entity_id, status, time_span, event_id).
fn collect_state_changes(chronicle: &Chronicle) -> Vec<(String, Status, TimeSpan, String)> {
    let mut changes = Vec::new();
    for nx in chronicle.graph.node_indices() {
        if let Entity::Event(event) = &chronicle.graph[nx] {
            for sc in &event.state_changes {
                if let StateChange::StatusChange(status) = &sc.change {
                    changes.push((
                        sc.entity.clone(),
                        status.clone(),
                        event.time_span.clone(),
                        event.id.clone(),
                    ));
                }
            }
        }
    }
    changes
}

// ── can_add_event ──────────────────────────────────────────

/// Check whether a proposed event is consistent with the existing graph.
/// Returns Ok(()) if the event can be added, or Err with all detected violations.
pub fn can_add_event(chronicle: &Chronicle, event: &Event) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    // Duplicate ID check
    if chronicle.index.contains_key(&event.id) {
        errors.push(ValidationError::DanglingReference {
            source_id: event.id.clone(),
            target_id: event.id.clone(),
            context: format!("proposed event '{}' has the same ID as an existing entity", event.id),
        });
    }

    // Referential: all referenced entities must exist
    if let Some(loc) = &event.location
        && !chronicle.index.contains_key(loc.as_str())
    {
        errors.push(ValidationError::DanglingReference {
            source_id: event.id.clone(),
            target_id: loc.clone(),
            context: "location".to_owned(),
        });
    }
    for cause_id in &event.caused_by {
        if !chronicle.index.contains_key(cause_id.as_str()) {
            errors.push(ValidationError::DanglingReference {
                source_id: event.id.clone(),
                target_id: cause_id.clone(),
                context: "caused_by".to_owned(),
            });
        }
    }
    for p in &event.participants {
        if !chronicle.index.contains_key(p.actor.as_str()) {
            errors.push(ValidationError::DanglingReference {
                source_id: event.id.clone(),
                target_id: p.actor.clone(),
                context: "participant".to_owned(),
            });
        }
    }
    for sc in &event.state_changes {
        if !chronicle.index.contains_key(sc.entity.as_str()) {
            errors.push(ValidationError::DanglingReference {
                source_id: event.id.clone(),
                target_id: sc.entity.clone(),
                context: "state_change target".to_owned(),
            });
        }
    }

    // Temporal: lifespan checks for participants
    if let Some(event_iv) = to_interval(&event.time_span) {
        for p in &event.participants {
            if let Some(&actor_nx) = chronicle.index.get(&p.actor)
                && let Entity::Actor(actor) = &chronicle.graph[actor_nx]
                && let Some(lifespan) = &actor.lifespan
                && let Some(life_iv) = to_interval(lifespan)
            {
                if strictly_before(&event_iv, &life_iv) {
                    errors.push(ValidationError::TemporalViolation {
                        entity_id: actor.id.clone(),
                        event_id: event.id.clone(),
                        description: format!(
                            "proposed event '{}' (year {}-{}) precedes actor '{}' lifespan ({}-{})",
                            event.id, event.time_span.start, event.time_span.end,
                            actor.id, lifespan.start, lifespan.end,
                        ),
                    });
                }
                if strictly_before(&life_iv, &event_iv) {
                    errors.push(ValidationError::TemporalViolation {
                        entity_id: actor.id.clone(),
                        event_id: event.id.clone(),
                        description: format!(
                            "actor '{}' lifespan ({}-{}) ends before proposed event '{}' (year {}-{})",
                            actor.id, lifespan.start, lifespan.end,
                            event.id, event.time_span.start, event.time_span.end,
                        ),
                    });
                }
            }
        }

        // Temporal: causal ordering
        for cause_id in &event.caused_by {
            if let Some(&cause_nx) = chronicle.index.get(cause_id.as_str())
                && let Entity::Event(cause_event) = &chronicle.graph[cause_nx]
                && let Some(cause_iv) = to_interval(&cause_event.time_span)
                && strictly_before(&event_iv, &cause_iv)
            {
                errors.push(ValidationError::TemporalViolation {
                    entity_id: event.id.clone(),
                    event_id: cause_event.id.clone(),
                    description: format!(
                        "proposed event '{}' (year {}-{}) precedes its cause '{}' (year {}-{})",
                        event.id, event.time_span.start, event.time_span.end,
                        cause_event.id, cause_event.time_span.start, cause_event.time_span.end,
                    ),
                });
            }
        }

        // State: check participants and location against terminal statuses
        let state_changes = collect_state_changes(chronicle);

        for p in &event.participants {
            for (entity_id, status, change_time, change_event_id) in &state_changes {
                if *entity_id == p.actor
                    && chronicle.config.terminal_statuses.contains(status)
                    && let Some(change_iv) = to_interval(change_time)
                    && strictly_before(&change_iv, &event_iv)
                {
                    errors.push(ValidationError::StateViolation {
                        entity_id: p.actor.clone(),
                        event_id: event.id.clone(),
                        description: format!(
                            "actor '{}' has status {:?} as of '{}' (year {}-{}), \
                             cannot participate in proposed event '{}' (year {}-{})",
                            p.actor, status, change_event_id,
                            change_time.start, change_time.end,
                            event.id, event.time_span.start, event.time_span.end,
                        ),
                    });
                }
            }
        }

        if let Some(loc) = &event.location {
            for (entity_id, status, change_time, change_event_id) in &state_changes {
                if entity_id == loc.as_str()
                    && chronicle.config.terminal_statuses.contains(status)
                    && let Some(change_iv) = to_interval(change_time)
                    && strictly_before(&change_iv, &event_iv)
                {
                    errors.push(ValidationError::StateViolation {
                        entity_id: loc.clone(),
                        event_id: event.id.clone(),
                        description: format!(
                            "location '{}' has status {:?} as of '{}' (year {}-{}), \
                             cannot host proposed event '{}' (year {}-{})",
                            loc, status, change_event_id,
                            change_time.start, change_time.end,
                            event.id, event.time_span.start, event.time_span.end,
                        ),
                    });
                }
            }
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}
