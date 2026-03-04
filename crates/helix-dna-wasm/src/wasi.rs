use helix_dna_core::ResourceOp;

/// Describes a WASI import needed by a resource operation.
#[derive(Debug, Clone)]
pub struct WasiImport {
    pub module: &'static str,
    pub name: &'static str,
    /// Parameter types (as WASM ValType).
    pub params: Vec<wasm_encoder::ValType>,
    /// Result types.
    pub results: Vec<wasm_encoder::ValType>,
}

/// Map a `ResourceOp` to the WASI import it requires.
pub fn resource_op_import(op: &ResourceOp) -> WasiImport {
    match op {
        ResourceOp::FdWrite { .. } => WasiImport {
            module: "wasi_snapshot_preview1",
            name: "fd_write",
            params: vec![
                wasm_encoder::ValType::I32, // fd
                wasm_encoder::ValType::I32, // iovs pointer
                wasm_encoder::ValType::I32, // iovs_len
                wasm_encoder::ValType::I32, // nwritten pointer
            ],
            results: vec![wasm_encoder::ValType::I32], // errno
        },
        ResourceOp::FdRead { .. } => WasiImport {
            module: "wasi_snapshot_preview1",
            name: "fd_read",
            params: vec![
                wasm_encoder::ValType::I32, // fd
                wasm_encoder::ValType::I32, // iovs pointer
                wasm_encoder::ValType::I32, // iovs_len
                wasm_encoder::ValType::I32, // nread pointer
            ],
            results: vec![wasm_encoder::ValType::I32], // errno
        },
        ResourceOp::PathOpen { .. } => WasiImport {
            module: "wasi_snapshot_preview1",
            name: "path_open",
            params: vec![
                wasm_encoder::ValType::I32, // fd (dirfd)
                wasm_encoder::ValType::I32, // dirflags
                wasm_encoder::ValType::I32, // path pointer
                wasm_encoder::ValType::I32, // path_len
                wasm_encoder::ValType::I32, // oflags
                wasm_encoder::ValType::I64, // fs_rights_base
                wasm_encoder::ValType::I64, // fs_rights_inheriting
                wasm_encoder::ValType::I32, // fdflags
                wasm_encoder::ValType::I32, // opened_fd pointer
            ],
            results: vec![wasm_encoder::ValType::I32], // errno
        },
    }
}
