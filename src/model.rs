//! Core data model: entity types, relationships, and graph structure.

use serde::{Deserialize, Serialize};

// ── Entity ID ──────────────────────────────────────────────

/// Stable string identifier for any entity in the graph.
pub type EntityId = String;

// ── Time ───────────────────────────────────────────────────

/// A span of time. Both bounds are inclusive integer years.
/// For point events, start == end.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TimeSpan {
    pub start: i32,
    pub end: i32,
}

// ── Status ─────────────────────────────────────────────────

/// Current state of an entity, potentially changed by events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Active,
    Dead,
    Destroyed,
    Dissolved,
    Evacuated,
    Captured,
    Transformed,
    Corrupted,
    Unknown,
}

// ── Sentiment ──────────────────────────────────────────────

/// How a participant feels about an event they were involved in.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Sentiment {
    Triumphant,
    Vindicated,
    Dutiful,
    Determined,
    Righteous,
    Transformative,
    Neutral,
    Sorrowful,
    Fearful,
    Desperate,
    Defiant,
    Devastating,
    Resentful,
    Fractured,
}

// ── Participant Role ───────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Role {
    Attacker,
    Defender,
    Leader,
    Participant,
    Witness,
    Discoverer,
    Instigator,
    Victim,
    Mediator,
}

// ── Fidelity ───────────────────────────────────────────────

/// How closely a subjective account matches the objective graph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Fidelity {
    /// Matches objective graph exactly (e.g., archive-drone with intact records).
    Canonical,
    /// Omissions but no fabrication (e.g., eyewitness with incomplete view).
    Partial,
    /// Genuine misremembering (e.g., elderly NPC).
    Distorted,
    /// Deliberate spin, selective truth (e.g., faction propagandist).
    Biased,
    /// Intentional lies (e.g., trickster NPC, corrupted record).
    Fabricated,
    /// Damaged/degraded record (e.g., storm-damaged archive-drone).
    Corrupted,
}

// ── State Change ───────────────────────────────────────────

/// A change to an entity's state caused by an event.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StateChange {
    StatusChange(Status),
    Founded,
    AffiliationAdded(EntityId),
    AffiliationRemoved(EntityId),
}

// ── Event Participant ──────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub actor: EntityId,
    pub role: Role,
    pub sentiment: Sentiment,
}

// ── Event State Change Record ──────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStateChange {
    pub entity: EntityId,
    pub change: StateChange,
}

// ── Entity Types ───────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActorType {
    Character,
    Faction,
    Deity,
    Organization,
    Creature,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlaceType {
    Settlement,
    Region,
    Dungeon,
    Landmark,
    Ruin,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    Battle,
    Siege,
    Discovery,
    Political,
    Migration,
    Death,
    Founding,
    Catastrophe,
    Ritual,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConceptType {
    Religion,
    Technology,
    Artifact,
    Law,
    Tradition,
}

// ── Entities (RON-deserializable) ──────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    pub id: EntityId,
    pub name: String,
    #[serde(rename = "type")]
    pub actor_type: ActorType,
    #[serde(default = "default_status")]
    pub status: Status,
    #[serde(default)]
    pub status_since_event: Option<EntityId>,
    #[serde(default)]
    pub affiliations: Vec<EntityId>,
    #[serde(default)]
    pub lifespan: Option<TimeSpan>,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Place {
    pub id: EntityId,
    pub name: String,
    #[serde(rename = "type")]
    pub place_type: PlaceType,
    #[serde(default = "default_status")]
    pub status: Status,
    #[serde(default)]
    pub status_since_event: Option<EntityId>,
    #[serde(default)]
    pub region: Option<EntityId>,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: EntityId,
    pub name: String,
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub time_span: TimeSpan,
    #[serde(default)]
    pub location: Option<EntityId>,
    #[serde(default)]
    pub caused_by: Vec<EntityId>,
    #[serde(default)]
    pub participants: Vec<Participant>,
    #[serde(default)]
    pub state_changes: Vec<EventStateChange>,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    pub id: EntityId,
    pub name: String,
    #[serde(rename = "type")]
    pub concept_type: ConceptType,
    #[serde(default)]
    pub origin_event: Option<EntityId>,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: EntityId,
    pub source: EntityId,
    pub fidelity: Fidelity,
    #[serde(default)]
    pub event_refs: Vec<EntityId>,
    pub text: String,
}

fn default_status() -> Status {
    Status::Active
}
