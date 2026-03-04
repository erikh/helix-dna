pub use helix_dna_core::{
    Concept, ConceptId, ConceptKind, ConceptNetwork, ContentAddressedStore, EdgeKind,
    ExecutionError, Executor, NetworkError, NodeError, ResultOp, ResourceOp, Value,
};
pub use helix_dna_nlp::{Intent, ParseError, Resolve, ResolveError, Resolver, TokenizeError};
pub use helix_dna_wasm::{Compile, CompileError, Compiler};

/// Unified error type for the Helix-DNA pipeline.
#[derive(Debug, thiserror::Error)]
pub enum HelixDnaError {
    #[error("NLP resolve error: {0}")]
    Resolve(#[from] ResolveError),

    #[error("WASM compile error: {0}")]
    Compile(#[from] CompileError),
}

/// Compile a natural language command into WASM bytes.
///
/// This is the main entry point: it creates a concept network, resolves the
/// NL input into it, and compiles the graph to a WASM module.
///
/// The returned bytes are a valid WASM module exporting a `"main"` function.
pub fn compile(input: &str) -> Result<Vec<u8>, HelixDnaError> {
    HelixDna::new().compile(input)
}

/// Builder for configuring and running the Helix-DNA pipeline.
pub struct HelixDna {
    resolver: Resolver,
    compiler: Compiler,
}

impl HelixDna {
    pub fn new() -> Self {
        Self {
            resolver: Resolver,
            compiler: Compiler,
        }
    }

    /// Compile a natural language command into WASM bytes.
    pub fn compile(&self, input: &str) -> Result<Vec<u8>, HelixDnaError> {
        let mut network = ConceptNetwork::new();
        let root = self.resolver.resolve(input, &mut network)?;
        let wasm = self.compiler.compile(&network, root)?;
        Ok(wasm)
    }
}

impl Default for HelixDna {
    fn default() -> Self {
        Self::new()
    }
}
