use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChronicleError {
    #[error("entity not found: {0}")]
    EntityNotFound(String),

    #[error("duplicate entity id: {0}")]
    DuplicateId(String),

    #[error("dangling reference: {source_id} references unknown entity {target_id}")]
    DanglingReference {
        source_id: String,
        target_id: String,
    },

    #[error("temporal violation: {description}")]
    TemporalViolation { description: String },

    #[error("state violation: {description}")]
    StateViolation { description: String },

    #[error("parse error: {0}")]
    Parse(#[from] ron::error::SpannedError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
