use serde::{Deserialize, Serialize};
use std::fmt;

/// Content-addressed identifier for a concept, wrapping a blake3 hash.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ConceptId([u8; 32]);

impl ConceptId {
    /// Create a ConceptId by hashing the given bytes.
    pub fn from_bytes(data: &[u8]) -> Self {
        Self(blake3::hash(data).into())
    }

    /// Create a ConceptId from a raw 32-byte array.
    pub fn from_raw(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Return the raw bytes of this ID.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for ConceptId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConceptId({})", &hex(&self.0)[..16])
    }
}

impl fmt::Display for ConceptId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &hex(&self.0)[..16])
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// The kind of concept node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConceptKind {
    /// Pure computation — always succeeds.
    Result(ResultOp),
    /// Impure — requests external resources, can fail.
    Resource(ResourceOp),
}

/// Pure operations that always succeed given valid inputs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResultOp {
    /// Integer constant.
    Const(i64),
    /// Add inputs.
    Add,
    /// Subtract inputs.
    Sub,
    /// Multiply inputs.
    Mul,
    /// Divide inputs.
    Div,
    /// Compare inputs for equality.
    Eq,
    /// Identity — pass through.
    Identity,
}

/// Resource operations that may fail (map to WASI syscalls).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResourceOp {
    /// Write bytes to fd.
    FdWrite { fd: u32 },
    /// Read bytes from fd.
    FdRead { fd: u32 },
    /// Open a file path.
    PathOpen { path: String },
}

/// A concept in the network.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Concept {
    /// Content-addressed identifier.
    pub id: ConceptId,
    /// Human-readable name.
    pub name: String,
    /// The operation this concept represents.
    pub kind: ConceptKind,
}

impl Concept {
    /// Create a new concept. The ID is computed from the name and kind.
    pub fn new(name: impl Into<String>, kind: ConceptKind) -> Self {
        let name = name.into();
        let hash_input = format!("{name}:{kind:?}");
        let id = ConceptId::from_bytes(hash_input.as_bytes());
        Self { id, name, kind }
    }

    /// Whether this concept is pure (a Result node).
    pub fn is_pure(&self) -> bool {
        matches!(self.kind, ConceptKind::Result(_))
    }
}

/// Runtime value flowing between concept nodes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bytes(Vec<u8>),
    Bool(bool),
    Unit,
}

impl Value {
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(v) => Some(*v),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concept_id_deterministic() {
        let id1 = ConceptId::from_bytes(b"hello");
        let id2 = ConceptId::from_bytes(b"hello");
        assert_eq!(id1, id2);
    }

    #[test]
    fn concept_id_different_inputs() {
        let id1 = ConceptId::from_bytes(b"hello");
        let id2 = ConceptId::from_bytes(b"world");
        assert_ne!(id1, id2);
    }

    #[test]
    fn concept_hashing_deterministic() {
        let c1 = Concept::new("add", ConceptKind::Result(ResultOp::Add));
        let c2 = Concept::new("add", ConceptKind::Result(ResultOp::Add));
        assert_eq!(c1.id, c2.id);
    }

    #[test]
    fn concept_different_name_different_id() {
        let c1 = Concept::new("add", ConceptKind::Result(ResultOp::Add));
        let c2 = Concept::new("plus", ConceptKind::Result(ResultOp::Add));
        assert_ne!(c1.id, c2.id);
    }

    #[test]
    fn concept_is_pure() {
        let pure = Concept::new("add", ConceptKind::Result(ResultOp::Add));
        let impure = Concept::new("write", ConceptKind::Resource(ResourceOp::FdWrite { fd: 1 }));
        assert!(pure.is_pure());
        assert!(!impure.is_pure());
    }

    #[test]
    fn value_as_int() {
        assert_eq!(Value::Int(42).as_int(), Some(42));
        assert_eq!(Value::Bool(true).as_int(), None);
        assert_eq!(Value::Unit.as_int(), None);
    }
}
