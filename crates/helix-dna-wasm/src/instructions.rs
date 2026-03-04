use helix_dna_core::ResultOp;
use wasm_encoder::Instruction;

/// Map a `ResultOp` to a sequence of WASM instructions.
///
/// For `Const(n)`, emits `i64.const n`.
/// For binary ops (Add, Sub, Mul, Div), emits the corresponding i64 instruction.
/// Inputs are expected to already be on the stack.
pub fn result_op_instructions(op: &ResultOp) -> Vec<Instruction<'static>> {
    match op {
        ResultOp::Const(n) => vec![Instruction::I64Const(*n)],
        ResultOp::Add => vec![Instruction::I64Add],
        ResultOp::Sub => vec![Instruction::I64Sub],
        ResultOp::Mul => vec![Instruction::I64Mul],
        ResultOp::Div => vec![Instruction::I64DivS],
        ResultOp::Eq => vec![Instruction::I64Eq],
        ResultOp::Identity => vec![], // no-op, value passes through
    }
}
