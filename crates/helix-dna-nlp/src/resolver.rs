use helix_dna_core::{
    ConceptId, ConceptKind, ConceptNetwork, EdgeKind, ResultOp,
};

use crate::error::ResolveError;
use crate::parser::{parse, Arg, Intent};
use crate::Resolve;

/// Resolves natural language input into a concept network.
pub struct Resolver;

impl Resolve for Resolver {
    fn resolve(
        &self,
        input: &str,
        network: &mut ConceptNetwork,
    ) -> Result<ConceptId, ResolveError> {
        let intent = parse(input)?;
        resolve_intent(&intent, network)
    }
}

fn verb_to_op(verb: &str) -> Result<ResultOp, ResolveError> {
    match verb {
        "add" | "plus" | "sum" => Ok(ResultOp::Add),
        "sub" | "subtract" | "minus" => Ok(ResultOp::Sub),
        "mul" | "multiply" | "times" => Ok(ResultOp::Mul),
        "div" | "divide" => Ok(ResultOp::Div),
        "eq" | "equal" | "equals" => Ok(ResultOp::Eq),
        _ => Err(ResolveError::UnknownVerb(verb.to_string())),
    }
}

fn resolve_intent(
    intent: &Intent,
    network: &mut ConceptNetwork,
) -> Result<ConceptId, ResolveError> {
    let op = verb_to_op(&intent.verb)?;

    let expected_arity = match &op {
        ResultOp::Add | ResultOp::Sub | ResultOp::Mul | ResultOp::Div | ResultOp::Eq => 2,
        ResultOp::Const(_) | ResultOp::Identity => 1,
    };

    if intent.args.len() != expected_arity {
        return Err(ResolveError::ArityMismatch {
            verb: intent.verb.clone(),
            expected: expected_arity,
            got: intent.args.len(),
        });
    }

    // Create argument concept nodes.
    let arg_ids: Vec<ConceptId> = intent
        .args
        .iter()
        .enumerate()
        .map(|(i, arg)| match arg {
            Arg::Number(n) => {
                network.create_concept(format!("arg_{i}"), ConceptKind::Result(ResultOp::Const(*n)))
            }
            Arg::Word(w) => {
                // Treat word args as identity nodes for now.
                network.create_concept(
                    format!("arg_{i}_{w}"),
                    ConceptKind::Result(ResultOp::Identity),
                )
            }
        })
        .collect();

    // Create the operation node.
    let op_id = network.create_concept(&intent.verb, ConceptKind::Result(op));

    // Link arguments as inputs to the operation.
    for (i, arg_id) in arg_ids.iter().enumerate() {
        network.link(*arg_id, op_id, EdgeKind::Input(i))?;
    }

    Ok(op_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use helix_dna_core::{Executor, Value};

    #[test]
    fn resolve_add_2_3() {
        let resolver = Resolver;
        let mut network = ConceptNetwork::new();
        let root = resolver.resolve("add 2 3", &mut network).unwrap();

        // Network should have 3 concepts: const(2), const(3), add
        assert_eq!(network.len(), 3);

        // Execute and verify
        let executor = Executor::new();
        let result = executor.execute(&network, root).unwrap();
        assert_eq!(result, vec![Value::Int(5)]);
    }

    #[test]
    fn resolve_subtract() {
        let resolver = Resolver;
        let mut network = ConceptNetwork::new();
        let root = resolver.resolve("sub 10 3", &mut network).unwrap();

        let executor = Executor::new();
        let result = executor.execute(&network, root).unwrap();
        assert_eq!(result, vec![Value::Int(7)]);
    }

    #[test]
    fn resolve_unknown_verb() {
        let resolver = Resolver;
        let mut network = ConceptNetwork::new();
        let err = resolver.resolve("fly 1 2", &mut network).unwrap_err();
        assert!(matches!(err, ResolveError::UnknownVerb(_)));
    }

    #[test]
    fn resolve_wrong_arity() {
        let resolver = Resolver;
        let mut network = ConceptNetwork::new();
        let err = resolver.resolve("add 1 2 3", &mut network).unwrap_err();
        assert!(matches!(err, ResolveError::ArityMismatch { .. }));
    }
}
