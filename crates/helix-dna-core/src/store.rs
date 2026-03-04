use std::collections::BTreeMap;

use crate::concept::{Concept, ConceptId};

/// Content-addressed store backed by a BTreeMap.
/// Two concepts that resolve identically share a hash and are stored once.
#[derive(Debug, Clone, Default)]
pub struct ContentAddressedStore {
    inner: BTreeMap<ConceptId, Concept>,
}

impl ContentAddressedStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a concept. If a concept with the same ID already exists,
    /// the existing one is returned (deduplication).
    pub fn insert(&mut self, concept: Concept) -> ConceptId {
        let id = concept.id;
        self.inner.entry(id).or_insert(concept);
        id
    }

    /// Look up a concept by its ID.
    pub fn get(&self, id: &ConceptId) -> Option<&Concept> {
        self.inner.get(id)
    }

    /// Check if a concept exists.
    pub fn contains(&self, id: &ConceptId) -> bool {
        self.inner.contains_key(id)
    }

    /// Number of stored concepts.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Whether the store is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Iterate over all stored concepts.
    pub fn iter(&self) -> impl Iterator<Item = (&ConceptId, &Concept)> {
        self.inner.iter()
    }

    /// Remove a concept by ID.
    pub fn remove(&mut self, id: &ConceptId) -> Option<Concept> {
        self.inner.remove(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concept::{ConceptKind, ResultOp};

    #[test]
    fn insert_and_get() {
        let mut store = ContentAddressedStore::new();
        let c = Concept::new("add", ConceptKind::Result(ResultOp::Add));
        let id = store.insert(c.clone());
        assert_eq!(store.get(&id).unwrap().name, "add");
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn deduplication() {
        let mut store = ContentAddressedStore::new();
        let c1 = Concept::new("add", ConceptKind::Result(ResultOp::Add));
        let c2 = Concept::new("add", ConceptKind::Result(ResultOp::Add));
        let id1 = store.insert(c1);
        let id2 = store.insert(c2);
        assert_eq!(id1, id2);
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn different_concepts_different_ids() {
        let mut store = ContentAddressedStore::new();
        let c1 = Concept::new("add", ConceptKind::Result(ResultOp::Add));
        let c2 = Concept::new("sub", ConceptKind::Result(ResultOp::Sub));
        let id1 = store.insert(c1);
        let id2 = store.insert(c2);
        assert_ne!(id1, id2);
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn remove_concept() {
        let mut store = ContentAddressedStore::new();
        let c = Concept::new("add", ConceptKind::Result(ResultOp::Add));
        let id = store.insert(c);
        assert!(store.contains(&id));
        store.remove(&id);
        assert!(!store.contains(&id));
        assert!(store.is_empty());
    }
}
