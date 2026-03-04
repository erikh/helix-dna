use wasmtime::{Engine, Instance, Module, Store};

#[test]
fn compile_add_2_3_returns_5() {
    let wasm_bytes = helix_dna::compile("add 2 3").expect("compile should succeed");

    // Validate the WASM bytes are parseable
    assert_eq!(&wasm_bytes[..4], b"\0asm");

    // Run with wasmtime
    let engine = Engine::default();
    let module = Module::new(&engine, &wasm_bytes).expect("module should load");
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[]).expect("instantiation should succeed");
    let main = instance
        .get_typed_func::<(), i64>(&mut store, "main")
        .expect("should have main export");
    let result = main.call(&mut store, ()).expect("main should succeed");
    assert_eq!(result, 5);
}

#[test]
fn compile_sub_10_3_returns_7() {
    let wasm_bytes = helix_dna::compile("sub 10 3").expect("compile should succeed");

    let engine = Engine::default();
    let module = Module::new(&engine, &wasm_bytes).expect("module should load");
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[]).expect("instantiation should succeed");
    let main = instance
        .get_typed_func::<(), i64>(&mut store, "main")
        .expect("should have main export");
    let result = main.call(&mut store, ()).expect("main should succeed");
    assert_eq!(result, 7);
}

#[test]
fn compile_multiply_4_5_returns_20() {
    let wasm_bytes = helix_dna::compile("mul 4 5").expect("compile should succeed");

    let engine = Engine::default();
    let module = Module::new(&engine, &wasm_bytes).expect("module should load");
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[]).expect("instantiation should succeed");
    let main = instance
        .get_typed_func::<(), i64>(&mut store, "main")
        .expect("should have main export");
    let result = main.call(&mut store, ()).expect("main should succeed");
    assert_eq!(result, 20);
}

#[test]
fn compile_unknown_verb_fails() {
    let result = helix_dna::compile("fly 1 2");
    assert!(result.is_err());
}

#[test]
fn helix_dna_builder_works() {
    let hdna = helix_dna::HelixDna::new();
    let wasm_bytes = hdna.compile("add 100 200").expect("compile should succeed");

    let engine = Engine::default();
    let module = Module::new(&engine, &wasm_bytes).expect("module should load");
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[]).expect("instantiation should succeed");
    let main = instance
        .get_typed_func::<(), i64>(&mut store, "main")
        .expect("should have main export");
    let result = main.call(&mut store, ()).expect("main should succeed");
    assert_eq!(result, 300);
}
