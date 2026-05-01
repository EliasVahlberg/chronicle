//! Graph builder: load RON files, resolve references, build StableGraph.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use petgraph::stable_graph::{NodeIndex, StableGraph};

use crate::error::ChronicleError;
use crate::model::*;
use crate::validation::{ValidationReport, ValidationError, validate, can_add_event};

/// The core chronicle graph.
///
/// Holds a heterogeneous [`StableGraph`] of [`Entity`] nodes connected by
/// [`Relationship`] edges, plus a string-to-index lookup map and a
/// [`ValidationConfig`] that governs policy decisions such as which statuses
/// are considered terminal.
///
/// Construct via [`Chronicle::from_directory`] (default config) or
/// [`Chronicle::from_directory_with_config`] (custom config). Both load every
/// `.ron` file under the conventional subdirectory layout (`actors/`, `places/`,
/// `events/`, `concepts/`, `accounts/`), insert nodes, and wire up edges in a
/// single pass.
pub struct Chronicle {
    /// The underlying directed graph. Nodes are [`Entity`] variants; edges are
    /// [`Relationship`] variants that encode the kind and, where applicable,
    /// the payload (role, sentiment, state-change) of each connection.
    pub graph: StableGraph<Entity, Relationship>,

    /// Maps every entity's string id to its [`NodeIndex`] in the graph,
    /// providing O(1) lookup by id.
    pub index: HashMap<String, NodeIndex>,

    /// Policy configuration used during validation — controls which
    /// [`Status`] values are treated as terminal (e.g. `Dead`, `Destroyed`).
    pub config: ValidationConfig,
}

impl Chronicle {
    /// Load all RON files from a directory, build the graph with default config.
    ///
    /// Equivalent to calling [`from_directory_with_config`](Self::from_directory_with_config)
    /// with [`ValidationConfig::default()`].
    ///
    /// # Errors
    ///
    /// Returns [`ChronicleError`] on I/O failures, RON parse errors, or
    /// duplicate entity ids across files.
    pub fn from_directory(path: &Path) -> Result<Self, ChronicleError> {
        Self::from_directory_with_config(path, ValidationConfig::default())
    }

    /// Load all RON files from a directory with a custom [`ValidationConfig`].
    ///
    /// Walks the conventional subdirectories (`actors/`, `places/`, `events/`,
    /// `concepts/`, `accounts/`), deserialises every `.ron` file into the
    /// corresponding entity type, inserts nodes, and then builds all edges in
    /// a separate pass via `build_edges`.
    ///
    /// # Errors
    ///
    /// Returns [`ChronicleError`] on I/O failures, RON parse errors, or
    /// duplicate entity ids.
    pub fn from_directory_with_config(
        path: &Path,
        config: ValidationConfig,
    ) -> Result<Self, ChronicleError> {
        let mut graph = StableGraph::new();
        let mut index = HashMap::new();

        load_dir::<Actor>(path, "actors", &mut graph, &mut index)?;
        load_dir::<Place>(path, "places", &mut graph, &mut index)?;
        load_dir::<Event>(path, "events", &mut graph, &mut index)?;
        load_dir::<Concept>(path, "concepts", &mut graph, &mut index)?;
        load_dir::<Account>(path, "accounts", &mut graph, &mut index)?;

        let mut chronicle = Self { graph, index, config };
        chronicle.build_edges();
        Ok(chronicle)
    }

    /// Run all validation passes and return a [`ValidationReport`].
    ///
    /// The report aggregates referential-integrity, temporal-consistency,
    /// state-validity, and orphan-detection errors found in the graph.
    pub fn validate(&self) -> ValidationReport {
        validate(self)
    }

    /// Check whether a proposed [`Event`] is consistent with the existing graph.
    ///
    /// Returns `Ok(())` if the event can be inserted without violating any
    /// temporal or state constraints. Returns `Err` with a list of
    /// [`ValidationError`]s describing every conflict found.
    pub fn can_add_event(&self, event: &Event) -> Result<(), Vec<ValidationError>> {
        can_add_event(self, event)
    }

    /// Build all edges from the entity data already in the graph.
    fn build_edges(&mut self) {
        // Collect edge data first to avoid borrow conflicts.
        let mut edges: Vec<(NodeIndex, String, Relationship)> = Vec::new();

        for nx in self.graph.node_indices() {
            let entity = &self.graph[nx];
            match entity {
                Entity::Actor(actor) => {
                    for aff in &actor.affiliations {
                        edges.push((nx, aff.clone(), Relationship::AffiliatedWith));
                    }
                }
                Entity::Event(event) => {
                    if let Some(loc) = &event.location {
                        edges.push((nx, loc.clone(), Relationship::OccurredAt));
                    }
                    for cause in &event.caused_by {
                        edges.push((nx, cause.clone(), Relationship::CausedBy));
                    }
                    for p in &event.participants {
                        edges.push((
                            nx,
                            p.actor.clone(),
                            Relationship::ParticipatedIn {
                                role: p.role.clone(),
                                sentiment: p.sentiment.clone(),
                            },
                        ));
                    }
                    for sc in &event.state_changes {
                        edges.push((
                            nx,
                            sc.entity.clone(),
                            Relationship::HasStateChange {
                                change: sc.change.clone(),
                            },
                        ));
                    }
                }
                Entity::Concept(concept) => {
                    if let Some(origin) = &concept.origin_event {
                        edges.push((nx, origin.clone(), Relationship::OriginatedFrom));
                    }
                }
                Entity::Account(account) => {
                    edges.push((
                        nx,
                        account.source.clone(),
                        Relationship::AuthoredBy,
                    ));
                    for ev in &account.event_refs {
                        edges.push((nx, ev.clone(), Relationship::AccountOf));
                    }
                    for mention in parse_references(&account.text) {
                        edges.push((nx, mention, Relationship::Mentions));
                    }
                }
                Entity::Place(place) => {
                    if let Some(r) = &place.region {
                        edges.push((nx, r.clone(), Relationship::LocatedIn));
                    }
                }
            }
        }

        for (from, target_id, rel) in edges {
            if let Some(&to) = self.index.get(&target_id) {
                self.graph.add_edge(from, to, rel);
            }
            // Dangling references are caught by validation, not here.
        }
    }
}

/// Extract `{entity_id}` references from account text.
///
/// Scans `text` for brace-delimited identifiers (e.g. `{silica}`) and returns
/// the collected ids as a `Vec<String>`. Empty braces (`{}`) are ignored.
///
/// # Examples
///
/// ```
/// use chronicle::graph::parse_references;
///
/// let refs = parse_references("We burned {silica}. The {mirror_order} call it a massacre.");
/// assert_eq!(refs, vec!["silica".to_string(), "mirror_order".to_string()]);
/// ```
pub fn parse_references(text: &str) -> Vec<String> {
    let mut refs = Vec::new();
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '{' {
            let id: String = chars.by_ref().take_while(|&c| c != '}').collect();
            if !id.is_empty() {
                refs.push(id);
            }
        }
    }
    refs
}

// ── Loading helpers ────────────────────────────────────────

trait IntoEntity {
    fn into_entity(self) -> Entity;
}

impl IntoEntity for Actor {
    fn into_entity(self) -> Entity { Entity::Actor(self) }
}
impl IntoEntity for Place {
    fn into_entity(self) -> Entity { Entity::Place(self) }
}
impl IntoEntity for Event {
    fn into_entity(self) -> Entity { Entity::Event(self) }
}
impl IntoEntity for Concept {
    fn into_entity(self) -> Entity { Entity::Concept(self) }
}
impl IntoEntity for Account {
    fn into_entity(self) -> Entity { Entity::Account(self) }
}

trait HasId {
    fn id(&self) -> &str;
}

impl HasId for Actor { fn id(&self) -> &str { &self.id } }
impl HasId for Place { fn id(&self) -> &str { &self.id } }
impl HasId for Event { fn id(&self) -> &str { &self.id } }
impl HasId for Concept { fn id(&self) -> &str { &self.id } }
impl HasId for Account { fn id(&self) -> &str { &self.id } }

fn load_dir<T>(
    root: &Path,
    subdir: &str,
    graph: &mut StableGraph<Entity, Relationship>,
    index: &mut HashMap<String, NodeIndex>,
) -> Result<(), ChronicleError>
where
    T: serde::de::DeserializeOwned + IntoEntity + HasId,
{
    let dir = root.join(subdir);
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "ron") {
            let contents = fs::read_to_string(&path)?;
            let items: Vec<T> = ron::from_str(&contents)?;
            for item in items {
                let id = item.id().to_owned();
                if index.contains_key(&id) {
                    return Err(ChronicleError::DuplicateId(id));
                }
                let nx = graph.add_node(item.into_entity());
                index.insert(id, nx);
            }
        }
    }
    Ok(())
}
