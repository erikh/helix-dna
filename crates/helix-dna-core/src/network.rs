use std::collections::HashMap;

use petgraph::stable_graph::{NodeIndex, StableDiGraph};
use petgraph::visit::EdgeRef;
use petgraph::Direction;

use crate::concept::{Concept, ConceptId, ConceptKind};
use crate::error::NetworkError;
use crate::store::ContentAddressedStore;

/// Edge label in the concept network.
#[derive(Debug, Clone, PartialEq)]
pub enum EdgeKind {
    /// Input dependency — the source provides input to the target.
    Input(usize),
    /// Fallback — used during remapping when the primary edge fails.
    Fallback,
}

/// The concept network: a directed graph of concepts with content-addressed storage.
#[derive(Debug, Clone)]
pub struct ConceptNetwork {
    store: ContentAddressedStore,
    graph: StableDiGraph<ConceptId, EdgeKind>,
    /// Map from ConceptId to graph node index for fast lookup.
    index_map: HashMap<ConceptId, NodeIndex>,
}

impl ConceptNetwork {
    pub fn new() -> Self {
        Self {
            store: ContentAddressedStore::new(),
            graph: StableDiGraph::new(),
            index_map: HashMap::new(),
        }
    }

    /// Add a concept to the network. Returns its ID.
    pub fn add_concept(&mut self, concept: Concept) -> ConceptId {
        let id = self.store.insert(concept);
        if !self.index_map.contains_key(&id) {
            let idx = self.graph.add_node(id);
            self.index_map.insert(id, idx);
        }
        id
    }

    /// Convenience: create and add a concept in one step.
    pub fn create_concept(&mut self, name: impl Into<String>, kind: ConceptKind) -> ConceptId {
        let concept = Concept::new(name, kind);
        self.add_concept(concept)
    }

    /// Link two concepts: `from` provides input to `to` at the given input index.
    pub fn link(
        &mut self,
        from: ConceptId,
        to: ConceptId,
        edge: EdgeKind,
    ) -> Result<(), NetworkError> {
        let from_idx = self
            .index_map
            .get(&from)
            .copied()
            .ok_or(NetworkError::ConceptNotFound(from))?;
        let to_idx = self
            .index_map
            .get(&to)
            .copied()
            .ok_or(NetworkError::ConceptNotFound(to))?;

        self.graph.add_edge(from_idx, to_idx, edge);
        Ok(())
    }

    /// Get a concept by ID.
    pub fn get_concept(&self, id: &ConceptId) -> Option<&Concept> {
        self.store.get(id)
    }

    /// Get the input concepts for a given concept (nodes with edges pointing to it),
    /// ordered by input index.
    pub fn inputs_of(&self, id: &ConceptId) -> Vec<ConceptId> {
        let Some(&idx) = self.index_map.get(id) else {
            return Vec::new();
        };

        let mut inputs: Vec<(usize, ConceptId)> = self
            .graph
            .edges_directed(idx, Direction::Incoming)
            .filter_map(|edge| {
                if let EdgeKind::Input(i) = edge.weight() {
                    Some((*i, self.graph[edge.source()]))
                } else {
                    None
                }
            })
            .collect();

        inputs.sort_by_key(|(i, _)| *i);
        inputs.into_iter().map(|(_, id)| id).collect()
    }

    /// Get fallback concepts for a given concept.
    pub fn fallbacks_of(&self, id: &ConceptId) -> Vec<ConceptId> {
        let Some(&idx) = self.index_map.get(id) else {
            return Vec::new();
        };

        self.graph
            .edges_directed(idx, Direction::Incoming)
            .filter_map(|edge| {
                if matches!(edge.weight(), EdgeKind::Fallback) {
                    Some(self.graph[edge.source()])
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get the outgoing targets of a concept (concepts it feeds into).
    pub fn outputs_of(&self, id: &ConceptId) -> Vec<ConceptId> {
        let Some(&idx) = self.index_map.get(id) else {
            return Vec::new();
        };

        self.graph
            .edges_directed(idx, Direction::Outgoing)
            .map(|edge| self.graph[edge.target()])
            .collect()
    }

    /// Access the underlying store.
    pub fn store(&self) -> &ContentAddressedStore {
        &self.store
    }

    /// Number of concepts in the network.
    pub fn len(&self) -> usize {
        self.store.len()
    }

    /// Whether the network is empty.
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    /// Relink an edge: remove the edge from `old_source` to `target` and add one
    /// from `new_source` to `target`. Used during remapping on failure.
    pub fn relink(
        &mut self,
        target: ConceptId,
        old_source: ConceptId,
        new_source: ConceptId,
        edge: EdgeKind,
    ) -> Result<(), NetworkError> {
        let target_idx = self
            .index_map
            .get(&target)
            .copied()
            .ok_or(NetworkError::ConceptNotFound(target))?;
        let old_idx = self
            .index_map
            .get(&old_source)
            .copied()
            .ok_or(NetworkError::ConceptNotFound(old_source))?;
        let new_idx = self
            .index_map
            .get(&new_source)
            .copied()
            .ok_or(NetworkError::ConceptNotFound(new_source))?;

        // Remove old edge
        if let Some(edge_idx) = self.graph.find_edge(old_idx, target_idx) {
            self.graph.remove_edge(edge_idx);
        }

        // Add new edge
        self.graph.add_edge(new_idx, target_idx, edge);
        Ok(())
    }
}

impl Default for ConceptNetwork {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concept::ResultOp;

    fn make_add_network() -> (ConceptNetwork, ConceptId, ConceptId, ConceptId) {
        let mut net = ConceptNetwork::new();
        let a = net.create_concept("a", ConceptKind::Result(ResultOp::Const(2)));
        let b = net.create_concept("b", ConceptKind::Result(ResultOp::Const(3)));
        let add = net.create_concept("add", ConceptKind::Result(ResultOp::Add));
        net.link(a, add, EdgeKind::Input(0)).unwrap();
        net.link(b, add, EdgeKind::Input(1)).unwrap();
        (net, a, b, add)
    }

    #[test]
    fn add_and_retrieve_concept() {
        let mut net = ConceptNetwork::new();
        let id = net.create_concept("foo", ConceptKind::Result(ResultOp::Const(1)));
        assert!(net.get_concept(&id).is_some());
        assert_eq!(net.len(), 1);
    }

    #[test]
    fn link_and_query_inputs() {
        let (net, a, b, add) = make_add_network();
        let inputs = net.inputs_of(&add);
        assert_eq!(inputs.len(), 2);
        assert_eq!(inputs[0], a);
        assert_eq!(inputs[1], b);
    }

    #[test]
    fn outputs_of() {
        let (net, a, _, add) = make_add_network();
        let outputs = net.outputs_of(&a);
        assert_eq!(outputs, vec![add]);
    }

    #[test]
    fn dedup_in_graph() {
        let mut net = ConceptNetwork::new();
        let id1 = net.create_concept("x", ConceptKind::Result(ResultOp::Const(1)));
        let id2 = net.create_concept("x", ConceptKind::Result(ResultOp::Const(1)));
        assert_eq!(id1, id2);
        assert_eq!(net.len(), 1);
    }

    #[test]
    fn executor_add_2_3() {
        let (net, _, _, add) = make_add_network();
        let executor = crate::execution::Executor::new();
        let result = executor.execute(&net, add).unwrap();
        assert_eq!(result, vec![crate::concept::Value::Int(5)]);
    }

    #[test]
    fn executor_nested_expression() {
        // (2 + 3) * 4
        let mut net = ConceptNetwork::new();
        let a = net.create_concept("a", ConceptKind::Result(ResultOp::Const(2)));
        let b = net.create_concept("b", ConceptKind::Result(ResultOp::Const(3)));
        let add = net.create_concept("add", ConceptKind::Result(ResultOp::Add));
        net.link(a, add, EdgeKind::Input(0)).unwrap();
        net.link(b, add, EdgeKind::Input(1)).unwrap();

        let c = net.create_concept("c", ConceptKind::Result(ResultOp::Const(4)));
        let mul = net.create_concept("mul", ConceptKind::Result(ResultOp::Mul));
        net.link(add, mul, EdgeKind::Input(0)).unwrap();
        net.link(c, mul, EdgeKind::Input(1)).unwrap();

        let executor = crate::execution::Executor::new();
        let result = executor.execute(&net, mul).unwrap();
        assert_eq!(result, vec![crate::concept::Value::Int(20)]);
    }
}
