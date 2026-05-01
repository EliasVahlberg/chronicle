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
    /// First year of the span (inclusive).
    pub start: i32,
    /// Last year of the span (inclusive).
    pub end: i32,
}

// ── Status ─────────────────────────────────────────────────

/// Current state of an entity, potentially changed by events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Status {
    /// Entity is alive / operational / intact.
    Active,
    /// Character has died (terminal by default).
    Dead,
    /// Place has been destroyed (terminal by default).
    Destroyed,
    /// Faction or organization has been dissolved (terminal by default).
    Dissolved,
    /// Place has been evacuated but still exists.
    Evacuated,
    /// Entity has been captured by another force.
    Captured,
    /// Entity has fundamentally changed form.
    Transformed,
    /// Entity has been corrupted or tainted.
    Corrupted,
    /// Status is not yet established.
    Unknown,
}

// ── Sentiment ──────────────────────────────────────────────

/// How a participant feels about an event they were involved in.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Sentiment {
    /// Victorious elation.
    Triumphant,
    /// Justified by the outcome.
    Vindicated,
    /// Fulfilled obligation without strong emotion.
    Dutiful,
    /// Resolved to continue despite difficulty.
    Determined,
    /// Morally certain of the cause.
    Righteous,
    /// Fundamentally changed by the experience.
    Transformative,
    /// No strong feeling either way.
    Neutral,
    /// Grief or mourning.
    Sorrowful,
    /// Overcome by fear.
    Fearful,
    /// Acting out of last-resort urgency.
    Desperate,
    /// Resisting despite the odds.
    Defiant,
    /// Emotionally shattered by the event.
    Devastating,
    /// Bitter anger toward the outcome or participants.
    Resentful,
    /// Internally divided or broken by the event.
    Fractured,
}

// ── Participant Role ───────────────────────────────────────

/// The role an actor plays in an event.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Role {
    /// Initiated or led an offensive action.
    Attacker,
    /// Resisted or protected against an action.
    Defender,
    /// Commanded or directed the event.
    Leader,
    /// Took part without a specialized role.
    Participant,
    /// Observed without direct involvement.
    Witness,
    /// Made a discovery or revelation.
    Discoverer,
    /// Provoked or triggered the event.
    Instigator,
    /// Suffered harm as a result of the event.
    Victim,
    /// Attempted to resolve conflict between parties.
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
    /// Entity transitions to a new [`Status`].
    StatusChange(Status),
    /// Entity is newly created or established.
    Founded,
    /// Entity gains an affiliation with the referenced entity.
    AffiliationAdded(EntityId),
    /// Entity loses an affiliation with the referenced entity.
    AffiliationRemoved(EntityId),
}

// ── Event Participant ──────────────────────────────────────

/// An actor's participation in an event, with role and emotional response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    /// The actor who participated.
    pub actor: EntityId,
    /// What role the actor played.
    pub role: Role,
    /// How the actor felt about the event.
    pub sentiment: Sentiment,
}

// ── Event State Change Record ──────────────────────────────

/// Links a [`StateChange`] to the entity it affects within an event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStateChange {
    /// The entity whose state changed.
    pub entity: EntityId,
    /// The change that occurred.
    pub change: StateChange,
}

// ── Entity Types ───────────────────────────────────────────

/// Classification of an [`Actor`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActorType {
    /// An individual person or named character.
    Character,
    /// A political, military, or ideological group.
    Faction,
    /// A divine or supernatural being.
    Deity,
    /// A structured group (guild, company, order).
    Organization,
    /// A non-humanoid living entity.
    Creature,
}

/// Classification of a [`Place`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlaceType {
    /// A city, town, or village.
    Settlement,
    /// A broad geographic area containing other places.
    Region,
    /// An underground or enclosed adventure site.
    Dungeon,
    /// A notable geographic or constructed feature.
    Landmark,
    /// A destroyed or abandoned former settlement.
    Ruin,
}

/// Classification of an [`Event`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    /// Open armed conflict.
    Battle,
    /// Prolonged encirclement or assault on a fortified place.
    Siege,
    /// Revelation of new knowledge or territory.
    Discovery,
    /// Governance, diplomacy, or power-structure change.
    Political,
    /// Large-scale movement of people.
    Migration,
    /// The death of a significant entity.
    Death,
    /// Establishment of a new settlement, faction, or institution.
    Founding,
    /// A large-scale disaster (natural or otherwise).
    Catastrophe,
    /// A magical, religious, or ceremonial event.
    Ritual,
}

/// Classification of a [`Concept`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConceptType {
    /// A belief system or faith.
    Religion,
    /// A technique, invention, or body of knowledge.
    Technology,
    /// A unique or significant object.
    Artifact,
    /// A codified rule or legal framework.
    Law,
    /// A cultural practice or custom.
    Tradition,
}

// ── Entities (RON-deserializable) ──────────────────────────

/// A character, faction, deity, organization, or creature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    /// Unique identifier for this actor.
    pub id: EntityId,
    /// Display name.
    pub name: String,
    /// What kind of actor this is.
    #[serde(rename = "type")]
    pub actor_type: ActorType,
    /// Current status (defaults to [`Status::Active`]).
    #[serde(default = "default_status")]
    pub status: Status,
    /// The event that caused the current status, if any.
    #[serde(default)]
    pub status_since_event: Option<EntityId>,
    /// Factions or organizations this actor belongs to.
    #[serde(default)]
    pub affiliations: Vec<EntityId>,
    /// Birth-to-death (or founding-to-dissolution) time span.
    #[serde(default)]
    pub lifespan: Option<TimeSpan>,
    /// Free-text description.
    #[serde(default)]
    pub description: String,
}

/// A settlement, region, dungeon, landmark, or ruin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Place {
    /// Unique identifier for this place.
    pub id: EntityId,
    /// Display name.
    pub name: String,
    /// What kind of place this is.
    #[serde(rename = "type")]
    pub place_type: PlaceType,
    /// Current status (defaults to [`Status::Active`]).
    #[serde(default = "default_status")]
    pub status: Status,
    /// The event that caused the current status, if any.
    #[serde(default)]
    pub status_since_event: Option<EntityId>,
    /// Parent region this place is located within.
    #[serde(default)]
    pub region: Option<EntityId>,
    /// Free-text description.
    #[serde(default)]
    pub description: String,
}

/// Something that happened: a battle, discovery, political shift, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Unique identifier for this event.
    pub id: EntityId,
    /// Display name.
    pub name: String,
    /// What kind of event this is.
    #[serde(rename = "type")]
    pub event_type: EventType,
    /// When the event occurred (inclusive year range).
    pub time_span: TimeSpan,
    /// Where the event took place, if applicable.
    #[serde(default)]
    pub location: Option<EntityId>,
    /// Events that caused or led to this one.
    #[serde(default)]
    pub caused_by: Vec<EntityId>,
    /// Actors involved and their roles.
    #[serde(default)]
    pub participants: Vec<Participant>,
    /// State changes this event inflicted on entities.
    #[serde(default)]
    pub state_changes: Vec<EventStateChange>,
    /// Free-text description.
    #[serde(default)]
    pub description: String,
}

/// An abstract idea: religion, technology, artifact, law, or tradition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    /// Unique identifier for this concept.
    pub id: EntityId,
    /// Display name.
    pub name: String,
    /// What kind of concept this is.
    #[serde(rename = "type")]
    pub concept_type: ConceptType,
    /// The event that introduced this concept, if any.
    #[serde(default)]
    pub origin_event: Option<EntityId>,
    /// Free-text description.
    #[serde(default)]
    pub description: String,
}

/// A subjective narrative fragment attributed to a source actor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Unique identifier for this account.
    pub id: EntityId,
    /// The actor who authored or narrated this account.
    pub source: EntityId,
    /// How reliable this account is relative to the objective graph.
    pub fidelity: Fidelity,
    /// Events this account describes or references.
    #[serde(default)]
    pub event_refs: Vec<EntityId>,
    /// The narrative text, potentially containing `{entity_id}` references.
    pub text: String,
}

fn default_status() -> Status {
    Status::Active
}

// ── Validation Config ──────────────────────────────────────

/// Policy configuration for validation rules.
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Statuses that prevent an entity from participating in future events.
    pub terminal_statuses: std::collections::HashSet<Status>,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            terminal_statuses: [Status::Dead, Status::Destroyed, Status::Dissolved].into(),
        }
    }
}

// ── Graph Node / Edge Enums ────────────────────────────────

/// A node in the chronicle graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Entity {
    /// An actor node (character, faction, etc.).
    Actor(Actor),
    /// A place node (settlement, region, etc.).
    Place(Place),
    /// An event node (battle, discovery, etc.).
    Event(Event),
    /// A concept node (religion, artifact, etc.).
    Concept(Concept),
    /// A subjective account node.
    Account(Account),
}

impl Entity {
    /// Returns the [`EntityId`] of the wrapped entity.
    pub fn id(&self) -> &str {
        match self {
            Entity::Actor(a) => &a.id,
            Entity::Place(p) => &p.id,
            Entity::Event(e) => &e.id,
            Entity::Concept(c) => &c.id,
            Entity::Account(a) => &a.id,
        }
    }
}

/// An edge in the chronicle graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Relationship {
    /// Actor → Event
    ParticipatedIn {
        /// The role the actor played in the event.
        role: Role,
        /// How the actor felt about the event.
        sentiment: Sentiment,
    },
    /// Event → Place
    OccurredAt,
    /// Event → Event (effect → cause)
    CausedBy,
    /// Event → affected Entity
    HasStateChange {
        /// The state change that was applied.
        change: StateChange,
    },
    /// Actor → Actor (member → faction)
    AffiliatedWith,
    /// Concept → Event
    OriginatedFrom,
    /// Account → Event
    AccountOf,
    /// Account → Actor (source)
    AuthoredBy,
    /// Account → any Entity (from {entity_id} refs in text)
    Mentions,
    /// Place → Place (child → parent region)
    LocatedIn,
}
