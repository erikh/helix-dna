use helix_dna_core::{ConceptId, ConceptKind, ConceptNetwork};
use wasm_encoder::Instruction;

use crate::error::CompileError;
use crate::instructions::result_op_instructions;

/// Walk the concept graph starting from `root` and emit WASM instructions
/// in dependency order (inputs before their consumers).
pub fn emit_instructions(
    network: &ConceptNetwork,
    root: ConceptId,
) -> Result<Vec<Instruction<'static>>, CompileError> {
    let mut instructions = Vec::new();
    emit_recursive(network, root, &mut instructions)?;
    Ok(instructions)
}

fn emit_recursive(
    network: &ConceptNetwork,
    id: ConceptId,
    instructions: &mut Vec<Instruction<'static>>,
) -> Result<(), CompileError> {
    let concept = network
        .get_concept(&id)
        .ok_or(CompileError::ConceptNotFound(id))?;

    // Emit inputs first (they push values onto the stack).
    let inputs = network.inputs_of(&id);
    for input_id in &inputs {
        emit_recursive(network, *input_id, instructions)?;
    }

    // Then emit this concept's instructions.
    match &concept.kind {
        ConceptKind::Result(op) => {
            instructions.extend(result_op_instructions(op));
        }
        ConceptKind::Resource(op) => {
            return Err(CompileError::UnsupportedOp(format!(
                "resource ops not yet supported in pure WASM codegen: {op:?}"
            )));
        }
    }

    Ok(())
}
