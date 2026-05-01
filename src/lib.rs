//! Event-centric narrative knowledge graphs with temporal verification.
//!
//! Chronicle loads authored narrative content into a typed, event-centric
//! knowledge graph, validates it for consistency, and provides a typed query API.
//!
//! # Quick Start
//!
//! ```no_run
//! use chronicle::graph::Chronicle;
//! use chronicle::model::*;
//!
//! // Load a world from RON files
//! let graph = Chronicle::from_directory("world/".as_ref())?;
//!
//! // Validate consistency
//! let report = graph.validate();
//! for error in &report.errors {
//!     eprintln!("{error}");
//! }
//!
//! // Typed queries
//! let kaine = graph.actor("kaine_durgan").unwrap();
//! let events = kaine.events();
//! let contacts = kaine.interactions().people();
//!
//! // Check if a proposed event is consistent
//! let proposed = Event {
//!     id: "new_battle".into(),
//!     name: "New Battle".into(),
//!     event_type: EventType::Battle,
//!     time_span: TimeSpan { start: 25, end: 25 },
//!     location: Some("silica".into()),
//!     caused_by: vec![],
//!     participants: vec![],
//!     state_changes: vec![],
//!     description: String::new(),
//! };
//! match graph.can_add_event(&proposed) {
//!     Ok(()) => println!("Event is consistent"),
//!     Err(errors) => {
//!         for e in &errors {
//!             eprintln!("{e}");
//!         }
//!     }
//! }
//! # Ok::<(), chronicle::error::ChronicleError>(())
//! ```

#![warn(missing_docs)]

pub mod error;
pub mod graph;
pub mod model;
pub mod query;
pub mod validation;
