use crate::concept::{ConceptId, ConceptKind, Value};
use crate::error::ExecutionError;
use crate::network::ConceptNetwork;
use crate::node::Evaluate;

/// Maximum number of remap attempts before giving up.
const MAX_REMAP_ATTEMPTS: usize = 10;

/// Executes concept chains in the network.
///
/// Walks the concept graph from a root concept, evaluating each node.
/// On resource node failure, attempts remapping via fallback edges.
pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Self
    }

    /// Execute the concept chain rooted at `root`, returning the result values.
    pub fn execute(
        &self,
        network: &ConceptNetwork,
        root: ConceptId,
    ) -> Result<Vec<Value>, ExecutionError> {
        self.execute_concept(network, root, 0)
    }

    fn execute_concept(
        &self,
        network: &ConceptNetwork,
        id: ConceptId,
        remap_depth: usize,
    ) -> Result<Vec<Value>, ExecutionError> {
        if remap_depth > MAX_REMAP_ATTEMPTS {
            return Err(ExecutionError::RemapExhausted(MAX_REMAP_ATTEMPTS));
        }

        let concept = network
            .get_concept(&id)
            .ok_or(ExecutionError::Network(
                crate::error::NetworkError::ConceptNotFound(id),
            ))?
            .clone();

        // Recursively evaluate inputs first.
        let input_ids = network.inputs_of(&id);
        let mut input_values = Vec::new();
        for input_id in &input_ids {
            let vals = self.execute_concept(network, *input_id, remap_depth)?;
            input_values.extend(vals);
        }

        // Evaluate this concept.
        let result = match &concept.kind {
            ConceptKind::Result(op) => op.evaluate(&input_values),
            ConceptKind::Resource(op) => op.evaluate(&input_values),
        };

        match result {
            Ok(values) => Ok(values),
            Err(e) => {
                // If this is a resource node and it failed, try fallbacks (remapping).
                if !concept.is_pure() {
                    let fallbacks = network.fallbacks_of(&id);
                    for fallback_id in fallbacks {
                        if let Ok(values) =
                            self.execute_concept(network, fallback_id, remap_depth + 1)
                        {
                            return Ok(values);
                        }
                    }
                }
                Err(ExecutionError::Node(e))
            }
        }
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}
