pub mod error;
pub mod parser;
pub mod resolver;
pub mod tokenizer;

pub use error::{ParseError, ResolveError, TokenizeError};
pub use parser::{Arg, Intent};
pub use resolver::Resolver;

use helix_dna_core::{ConceptId, ConceptNetwork};

/// Trait for resolving natural language input into a concept network.
pub trait Resolve {
    fn resolve(
        &self,
        input: &str,
        network: &mut ConceptNetwork,
    ) -> Result<ConceptId, ResolveError>;
}
