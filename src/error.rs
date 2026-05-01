//! Error types for the Chronicle crate.
//!
//! Defines [`ChronicleError`], the unified error enum returned by loading,
//! validation, and query operations.

use thiserror::Error;

/// Errors that can occur when loading, validating, or querying a Chronicle graph.
#[derive(Debug, Error)]
pub enum ChronicleError {
    /// A lookup by entity ID found no matching node in the graph.
    #[error("entity not found: {0}")]
    EntityNotFound(String),

    /// Two or more entities share the same ID, which must be unique.
    #[error("duplicate entity id: {0}")]
    DuplicateId(String),

    /// An entity references another entity ID that does not exist in the graph.
    #[error("dangling reference: {source_id} references unknown entity {target_id}")]
    DanglingReference {
        /// The ID of the entity that contains the broken reference.
        source_id: String,
        /// The ID that was referenced but not found.
        target_id: String,
    },

    /// A temporal constraint (Allen's Interval Algebra) is violated between events.
    #[error("temporal violation: {description}")]
    TemporalViolation {
        /// Human-readable explanation of the temporal inconsistency.
        description: String,
    },

    /// An event conflicts with the current state of a participant or location
    /// (e.g. a dead actor participating, or a destroyed place hosting an event).
    #[error("state violation: {description}")]
    StateViolation {
        /// Human-readable explanation of the state conflict.
        description: String,
    },

    /// A RON file could not be parsed.
    #[error("parse error: {0}")]
    Parse(#[from] ron::error::SpannedError),

    /// A filesystem I/O operation failed while reading world data.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
