#[derive(Debug, thiserror::Error)]
pub enum TokenizeError {
    #[error("failed to tokenize input: {0}")]
    PestError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("no verb found in input")]
    MissingVerb,

    #[error("tokenization failed: {0}")]
    Tokenize(#[from] TokenizeError),
}

#[derive(Debug, thiserror::Error)]
pub enum ResolveError {
    #[error("unknown verb: {0}")]
    UnknownVerb(String),

    #[error("wrong number of arguments for '{verb}': expected {expected}, got {got}")]
    ArityMismatch {
        verb: String,
        expected: usize,
        got: usize,
    },

    #[error("parse error: {0}")]
    Parse(#[from] ParseError),

    #[error("network error: {0}")]
    Network(#[from] helix_dna_core::NetworkError),
}
