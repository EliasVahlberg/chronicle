//! Typed query API: method chains over the chronicle graph.

use std::collections::HashSet;
use std::ops::RangeInclusive;

use petgraph::Direction;
use petgraph::stable_graph::NodeIndex;
use petgraph::visit::EdgeRef;

use crate::graph::Chronicle;
use crate::model::*;

// ── Edge-filtered neighbor helper ──────────────────────────

impl Chronicle {
    /// Nodes connected to `nx` via edges matching `pred`, in the given direction.
    fn neighbors_by_edge<F>(&self, nx: NodeIndex, dir: Direction, pred: F) -> Vec<NodeIndex>
    where
        F: Fn(&Relationship) -> bool,
    {
        self.graph
            .edges_directed(nx, dir)
            .filter(|e| pred(e.weight()))
            .map(|e| match dir {
                Direction::Outgoing => e.target(),
                Direction::Incoming => e.source(),
            })
            .collect()
    }
}

// ── Entry points on Chronicle ──────────────────────────────

impl Chronicle {
    pub fn actor(&self, id: &str) -> Option<ActorQuery<'_>> {
        let &nx = self.index.get(id)?;
        match &self.graph[nx] {
            Entity::Actor(_) => Some(ActorQuery { chronicle: self, nx }),
            _ => None,
        }
    }

    pub fn event(&self, id: &str) -> Option<EventQuery<'_>> {
        let &nx = self.index.get(id)?;
        match &self.graph[nx] {
            Entity::Event(_) => Some(EventQuery { chronicle: self, nx }),
            _ => None,
        }
    }

    pub fn place(&self, id: &str) -> Option<PlaceQuery<'_>> {
        let &nx = self.index.get(id)?;
        match &self.graph[nx] {
            Entity::Place(_) => Some(PlaceQuery { chronicle: self, nx }),
            _ => None,
        }
    }

    /// All accounts whose text contains a `{entity_id}` reference to this entity.
    pub fn mentions(&self, entity_id: &str) -> Vec<&Account> {
        let Some(&target_nx) = self.index.get(entity_id) else { return vec![] };
        let mut seen = HashSet::new();
        self.neighbors_by_edge(target_nx, Direction::Incoming, |r| {
            matches!(r, Relationship::Mentions)
        })
        .iter()
        .filter(|&&nx| seen.insert(nx))
        .filter_map(|&nx| match &self.graph[nx] {
            Entity::Account(a) => Some(a),
            _ => None,
        })
        .collect()
    }

    /// All accounts that reference this event (via AccountOf edges).
    pub fn accounts_of(&self, event_id: &str) -> Vec<&Account> {
        let Some(&target_nx) = self.index.get(event_id) else { return vec![] };
        let mut seen = HashSet::new();
        self.neighbors_by_edge(target_nx, Direction::Incoming, |r| {
            matches!(r, Relationship::AccountOf)
        })
        .iter()
        .filter(|&&nx| seen.insert(nx))
        .filter_map(|&nx| match &self.graph[nx] {
            Entity::Account(a) => Some(a),
            _ => None,
        })
        .collect()
    }

    /// All accounts authored by a given source.
    pub fn accounts_by(&self, source_id: &str) -> Vec<&Account> {
        let Some(&source_nx) = self.index.get(source_id) else { return vec![] };
        let mut seen = HashSet::new();
        self.neighbors_by_edge(source_nx, Direction::Incoming, |r| {
            matches!(r, Relationship::AuthoredBy)
        })
        .iter()
        .filter(|&&nx| seen.insert(nx))
        .filter_map(|&nx| match &self.graph[nx] {
            Entity::Account(a) => Some(a),
            _ => None,
        })
        .collect()
    }
}

// ── ActorQuery ─────────────────────────────────────────────

pub struct ActorQuery<'a> {
    chronicle: &'a Chronicle,
    nx: NodeIndex,
}

impl<'a> ActorQuery<'a> {
    pub fn data(&self) -> &'a Actor {
        match &self.chronicle.graph[self.nx] {
            Entity::Actor(a) => a,
            _ => unreachable!(),
        }
    }

    /// All events this actor participated in.
    pub fn events(&self) -> Vec<&'a Event> {
        self.chronicle
            .neighbors_by_edge(self.nx, Direction::Incoming, |r| {
                matches!(r, Relationship::ParticipatedIn { .. })
            })
            .iter()
            .filter_map(|&nx| match &self.chronicle.graph[nx] {
                Entity::Event(e) => Some(e),
                _ => None,
            })
            .collect()
    }

    /// Events filtered by time range.
    pub fn events_during(&self, range: RangeInclusive<i32>) -> Vec<&'a Event> {
        self.events()
            .into_iter()
            .filter(|e| e.time_span.start <= *range.end() && e.time_span.end >= *range.start())
            .collect()
    }

    /// Events filtered by location.
    pub fn events_at(&self, place_id: &str) -> Vec<&'a Event> {
        self.events()
            .into_iter()
            .filter(|e| e.location.as_deref() == Some(place_id))
            .collect()
    }

    /// All other actors this actor has co-participated in events with.
    pub fn interactions(&self) -> InteractionResult<'a> {
        let mut actors = Vec::new();
        for event in self.events() {
            for p in &event.participants {
                if p.actor != self.data().id
                    && let Some(q) = self.chronicle.actor(&p.actor)
                    && !actors.iter().any(|a: &&Actor| a.id == q.data().id)
                {
                    actors.push(q.data());
                }
            }
        }
        InteractionResult { actors }
    }

    /// Status at a given point in time, computed from state changes.
    pub fn status_at(&self, year: i32) -> Status {
        status_at_impl(self.chronicle, &self.data().id, year)
    }
}

// ── InteractionResult ──────────────────────────────────────

pub struct InteractionResult<'a> {
    actors: Vec<&'a Actor>,
}

impl<'a> InteractionResult<'a> {
    pub fn all(&self) -> &[&'a Actor] {
        &self.actors
    }

    pub fn people(&self) -> Vec<&'a Actor> {
        self.actors.iter().filter(|a| a.actor_type == ActorType::Character).copied().collect()
    }

    pub fn factions(&self) -> Vec<&'a Actor> {
        self.actors.iter().filter(|a| a.actor_type == ActorType::Faction).copied().collect()
    }
}

// ── EventQuery ─────────────────────────────────────────────

pub struct EventQuery<'a> {
    chronicle: &'a Chronicle,
    nx: NodeIndex,
}

impl<'a> EventQuery<'a> {
    pub fn data(&self) -> &'a Event {
        match &self.chronicle.graph[self.nx] {
            Entity::Event(e) => e,
            _ => unreachable!(),
        }
    }

    /// All participants in this event.
    pub fn participants(&self) -> Vec<&'a Participant> {
        self.data().participants.iter().collect()
    }

    /// Participants filtered by role.
    pub fn participants_by_role(&self, role: Role) -> Vec<&'a Participant> {
        self.data().participants.iter().filter(|p| p.role == role).collect()
    }

    /// The location of this event, if any.
    pub fn location(&self) -> Option<&'a Place> {
        let loc_id = self.data().location.as_ref()?;
        let &nx = self.chronicle.index.get(loc_id.as_str())?;
        match &self.chronicle.graph[nx] {
            Entity::Place(p) => Some(p),
            _ => None,
        }
    }

    /// Direct causes of this event.
    pub fn caused_by(&self) -> Vec<&'a Event> {
        self.data()
            .caused_by
            .iter()
            .filter_map(|id| {
                let &nx = self.chronicle.index.get(id.as_str())?;
                match &self.chronicle.graph[nx] {
                    Entity::Event(e) => Some(e),
                    _ => None,
                }
            })
            .collect()
    }

    /// Full causal chain backward (BFS through caused_by).
    pub fn causal_chain(&self) -> Vec<&'a Event> {
        let mut chain = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        let mut visited = HashSet::new();

        for cause in &self.data().caused_by {
            if let Some(&nx) = self.chronicle.index.get(cause.as_str()) {
                queue.push_back(nx);
            }
        }

        while let Some(nx) = queue.pop_front() {
            if !visited.insert(nx) {
                continue;
            }
            if let Entity::Event(e) = &self.chronicle.graph[nx] {
                chain.push(e);
                for cause_id in &e.caused_by {
                    if let Some(&cause_nx) = self.chronicle.index.get(cause_id.as_str()) {
                        queue.push_back(cause_nx);
                    }
                }
            }
        }
        chain
    }

    /// Events caused by this one (forward consequences via CausedBy edges).
    pub fn consequences(&self) -> Vec<&'a Event> {
        self.chronicle
            .neighbors_by_edge(self.nx, Direction::Incoming, |r| {
                matches!(r, Relationship::CausedBy)
            })
            .iter()
            .filter_map(|&nx| match &self.chronicle.graph[nx] {
                Entity::Event(e) => Some(e),
                _ => None,
            })
            .collect()
    }

    /// State changes produced by this event.
    pub fn state_changes(&self) -> &[EventStateChange] {
        &self.data().state_changes
    }
}

// ── PlaceQuery ─────────────────────────────────────────────

pub struct PlaceQuery<'a> {
    chronicle: &'a Chronicle,
    nx: NodeIndex,
}

impl<'a> PlaceQuery<'a> {
    pub fn data(&self) -> &'a Place {
        match &self.chronicle.graph[self.nx] {
            Entity::Place(p) => p,
            _ => unreachable!(),
        }
    }

    /// All events at this place.
    pub fn events(&self) -> Vec<&'a Event> {
        self.chronicle
            .neighbors_by_edge(self.nx, Direction::Incoming, |r| {
                matches!(r, Relationship::OccurredAt)
            })
            .iter()
            .filter_map(|&nx| match &self.chronicle.graph[nx] {
                Entity::Event(e) => Some(e),
                _ => None,
            })
            .collect()
    }

    /// Events at this place filtered by time range.
    pub fn events_during(&self, range: RangeInclusive<i32>) -> Vec<&'a Event> {
        self.events()
            .into_iter()
            .filter(|e| e.time_span.start <= *range.end() && e.time_span.end >= *range.start())
            .collect()
    }

    /// All actors who participated in events at this place during a given year.
    pub fn actors_present_at(&self, year: i32) -> Vec<&'a Actor> {
        let mut actors = Vec::new();
        for event in self.events() {
            if event.time_span.start <= year && event.time_span.end >= year {
                for p in &event.participants {
                    if let Some(q) = self.chronicle.actor(&p.actor)
                        && !actors.iter().any(|a: &&Actor| a.id == q.data().id)
                    {
                        actors.push(q.data());
                    }
                }
            }
        }
        actors
    }

    /// Status at a given point in time.
    pub fn status_at(&self, year: i32) -> Status {
        status_at_impl(self.chronicle, &self.data().id, year)
    }
}

// ── Shared helpers ─────────────────────────────────────────

/// Compute status at a point in time by scanning state changes.
/// Starts from Active and applies state changes up to the queried year in chronological order.
fn status_at_impl(chronicle: &Chronicle, entity_id: &str, year: i32) -> Status {
    let mut changes: Vec<(i32, &Status)> = Vec::new();

    for nx in chronicle.graph.node_indices() {
        if let Entity::Event(event) = &chronicle.graph[nx]
            && event.time_span.end <= year
        {
            for sc in &event.state_changes {
                if sc.entity == entity_id
                    && let StateChange::StatusChange(s) = &sc.change
                {
                    changes.push((event.time_span.end, s));
                }
            }
        }
    }

    changes.sort_by_key(|(time, _)| *time);
    changes.last().map(|(_, s)| (*s).clone()).unwrap_or(Status::Active)
}
