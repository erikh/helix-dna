use helix_dna_core::ConceptId;

#[derive(Debug, thiserror::Error)]
pub enum CompileError {
    #[error("concept not found: {0}")]
    ConceptNotFound(ConceptId),

    #[error("unsupported operation for WASM codegen: {0}")]
    UnsupportedOp(String),

    #[error("empty concept network — nothing to compile")]
    EmptyNetwork,

    #[error("WASM encoding error: {0}")]
    Encoding(String),
}
