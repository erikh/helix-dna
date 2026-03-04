use helix_dna_core::{ConceptId, ConceptNetwork};
use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction, Module,
    TypeSection, ValType,
};

use crate::codegen::emit_instructions;
use crate::error::CompileError;

/// Assemble a complete WASM module from a concept network rooted at `root`.
///
/// The generated module exports a single function `"main"` that takes no
/// parameters and returns a single i64 result.
pub fn build_module(
    network: &ConceptNetwork,
    root: ConceptId,
) -> Result<Vec<u8>, CompileError> {
    let instructions = emit_instructions(network, root)?;

    let mut module = Module::new();

    // Type section: one function type () -> (i64)
    let mut types = TypeSection::new();
    types.ty().function(vec![], vec![ValType::I64]);
    module.section(&types);

    // Function section: one function of type 0
    let mut functions = FunctionSection::new();
    functions.function(0);
    module.section(&functions);

    // Export section: export function 0 as "main"
    let mut exports = ExportSection::new();
    exports.export("main", ExportKind::Func, 0);
    module.section(&exports);

    // Code section: the function body
    let mut codes = CodeSection::new();
    let mut func = Function::new(vec![]);
    for instr in &instructions {
        func.instruction(instr);
    }
    func.instruction(&Instruction::End);
    codes.function(&func);
    module.section(&codes);

    Ok(module.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use helix_dna_core::{ConceptKind, EdgeKind, ResultOp};

    fn build_add_module(a: i64, b: i64) -> Vec<u8> {
        let mut net = ConceptNetwork::new();
        let ca = net.create_concept("a", ConceptKind::Result(ResultOp::Const(a)));
        let cb = net.create_concept("b", ConceptKind::Result(ResultOp::Const(b)));
        let add = net.create_concept("add", ConceptKind::Result(ResultOp::Add));
        net.link(ca, add, EdgeKind::Input(0)).unwrap();
        net.link(cb, add, EdgeKind::Input(1)).unwrap();
        build_module(&net, add).unwrap()
    }

    #[test]
    fn generated_wasm_is_valid() {
        let wasm = build_add_module(2, 3);
        wasmparser::validate(&wasm).expect("generated WASM should be valid");
    }

    #[test]
    fn generated_wasm_starts_with_magic() {
        let wasm = build_add_module(1, 1);
        assert_eq!(&wasm[..4], b"\0asm");
    }

    #[test]
    fn generated_wasm_subtract_is_valid() {
        let mut net = ConceptNetwork::new();
        let a = net.create_concept("a", ConceptKind::Result(ResultOp::Const(10)));
        let b = net.create_concept("b", ConceptKind::Result(ResultOp::Const(3)));
        let sub = net.create_concept("sub", ConceptKind::Result(ResultOp::Sub));
        net.link(a, sub, EdgeKind::Input(0)).unwrap();
        net.link(b, sub, EdgeKind::Input(1)).unwrap();
        let wasm = build_module(&net, sub).unwrap();
        wasmparser::validate(&wasm).expect("generated WASM should be valid");
    }
}
