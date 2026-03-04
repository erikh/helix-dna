pub mod codegen;
pub mod error;
pub mod instructions;
pub mod module;
pub mod wasi;

pub use error::CompileError;
pub use module::build_module;

use helix_dna_core::{ConceptId, ConceptNetwork};

/// Trait for compiling a concept network to WASM bytes.
pub trait Compile {
    fn compile(
        &self,
        network: &ConceptNetwork,
        root: ConceptId,
    ) -> Result<Vec<u8>, CompileError>;
}

/// The default WASM compiler.
pub struct Compiler;

impl Compile for Compiler {
    fn compile(
        &self,
        network: &ConceptNetwork,
        root: ConceptId,
    ) -> Result<Vec<u8>, CompileError> {
        build_module(network, root)
    }
}
