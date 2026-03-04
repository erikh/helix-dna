use crate::concept::ConceptId;

#[derive(Debug, thiserror::Error)]
pub enum NodeError {
    #[error("type mismatch: expected {expected}, got {got}")]
    TypeMismatch { expected: String, got: String },

    #[error("wrong number of inputs: expected {expected}, got {got}")]
    ArityMismatch { expected: usize, got: usize },

    #[error("resource unavailable: {0}")]
    ResourceUnavailable(String),

    #[error("division by zero")]
    DivisionByZero,
}

#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
    #[error("concept not found: {0}")]
    ConceptNotFound(ConceptId),

    #[error("edge already exists from {from} to {to}")]
    EdgeExists { from: ConceptId, to: ConceptId },

    #[error("cycle detected: adding edge from {from} to {to} would create a cycle")]
    CycleDetected { from: ConceptId, to: ConceptId },
}

#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("node error: {0}")]
    Node(#[from] NodeError),

    #[error("network error: {0}")]
    Network(#[from] NetworkError),

    #[error("max remap attempts ({0}) exhausted")]
    RemapExhausted(usize),

    #[error("no root concept to execute")]
    NoRoot,
}
