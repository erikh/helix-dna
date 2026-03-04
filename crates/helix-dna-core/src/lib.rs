pub mod concept;
pub mod error;
pub mod execution;
pub mod network;
pub mod node;
pub mod store;

pub use concept::{Concept, ConceptId, ConceptKind, ResultOp, ResourceOp, Value};
pub use error::{ExecutionError, NetworkError, NodeError};
pub use execution::Executor;
pub use network::{ConceptNetwork, EdgeKind};
pub use node::Evaluate;
pub use store::ContentAddressedStore;

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn concept_id_deterministic(s in "[a-z]{1,20}") {
            let c1 = Concept::new(&s, ConceptKind::Result(ResultOp::Const(1)));
            let c2 = Concept::new(&s, ConceptKind::Result(ResultOp::Const(1)));
            prop_assert_eq!(c1.id, c2.id);
        }

        #[test]
        fn executor_add_deterministic(a in -1000i64..1000, b in -1000i64..1000) {
            let mut net = ConceptNetwork::new();
            let ca = net.create_concept("a", ConceptKind::Result(ResultOp::Const(a)));
            let cb = net.create_concept("b", ConceptKind::Result(ResultOp::Const(b)));
            let add = net.create_concept("add", ConceptKind::Result(ResultOp::Add));
            net.link(ca, add, EdgeKind::Input(0)).unwrap();
            net.link(cb, add, EdgeKind::Input(1)).unwrap();

            let executor = Executor::new();
            let r1 = executor.execute(&net, add).unwrap();
            let r2 = executor.execute(&net, add).unwrap();
            prop_assert_eq!(&r1, &r2);
            prop_assert_eq!(r1[0].clone(), Value::Int(a + b));
        }
    }
}
