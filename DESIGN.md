# Helix-DNA - Concept Network Thought-to-Instruction Compiler

## Origin

A conversation exploring whether structured, deterministic concept networks could
replace statistical next-token prediction for compiling natural language thoughts
into executable instructions.

## Core Architecture

**Input:** Natural language (a "thought")
**Output:** A formal instruction / action description (targeting WASM/WASI)

The system is a **thought-to-instruction compiler**.

---

## Design Principles (in order they were established)

### 1. Concept Network (not a single tree)

Concepts associate with other concepts, forming chains of responsibility. Each
concept can be its own **subtree** that runs independently. Subtrees compose like
a network -- output of one feeds input of another. This is a **process network**,
not a tree. Each concept-subtree is a process with inputs flowing between them.

### 2. B-tree Storage (evolved from Merkle tree)

- Started with Merkle trees for content-addressing and integrity
- Evolved to B-trees because the system is a live network that's constantly
  remapping, routing, and deduplicating
- B-trees give: fast O(log n) lookup, ordered traversal, efficient insert/delete,
  natural deduplication, cache-friendliness
- **Keep the Merkle property** (content-addressed hashing) but store it **in** a
  B-tree -- keyed by concept hash
- Two concepts that resolve the same way have the same hash -> stored once,
  referenced everywhere

### 3. Two Types of Nodes

- **Result nodes (pure):** Given input, always produce output. Arithmetic,
  comparison, concept resolution. These are the backbone. They **cannot fail**.
- **Resource nodes (impure):** Ask for something external. Disk, network, memory
  allocation. These **can fail**. They're the boundary with reality. Map to WASI
  syscalls.

### 4. Execution Model (inspired by eBPF)

- **No separate verification step.** Execution IS verification.
- The instruction pipeline runs until it either succeeds or fails
- Syscalls are guaranteed to fail if a resource is unavailable -- that's the
  rejection signal
- Like eBPF: restricted execution, sandboxed, failure is just a return code
- Traditional: `generate -> verify -> accept/reject -> execute`
- This system: `generate -> execute (failure = rejection signal)`

### 5. Concepts ARE the Computation

- No explicit instruction set (no LOAD, ADD, STORE etc.)
- Concepts resolving to other concepts **is** computation
- A concept chain reaching a real-world boundary **is** a syscall
- A concept chain staying internal **is** a memory operation
- The distinction between "instruction" and "data" doesn't exist -- it's all
  **resolution depth**

### 6. Self-Remapping on Failure

- When a resource node fails, the tree remaps -- relink one edge
- The content-addressed hash propagates up from the changed node, marking exactly
  what changed
- Everything else stays verified/cached
- Remapping continues until a working solution is found

### 7. State Machine Determinism

- Each subtree is a deterministic state machine
- Inputs determine the path through variable instructions
- State machine determinism is tested/verified
- For any given input, a subtree always takes the same path and produces the same
  output

### 8. NLP Input Structure (ELF binary analogy)

The NLP component could be structured like an ELF binary:

- `.text`    -> executable transformation rules
- `.data`    -> mutable knowledge state
- `.rodata`  -> immutable facts / axioms
- `.symtab`  -> concept/token index (vocabulary)
- `.rel`     -> relocation tables (how contexts link)
- `.dynamic` -> runtime linking (connecting to new inputs)

A **self-describing, linkable knowledge artifact** where knowledge has sections
with different mutability guarantees.

### 9. Attention Mechanism

- Mixed with the concept network structure
- A "double matrix of sums" linked in as many ways as necessary
- Connections adjusted by other systems (looping over consistent I/O)
- Patterns pass through deterministic machines that verify success -- rejected if
  they fail
- The attention mechanism picks which associations to follow given input context
- It's navigating a **structured, verified space of known-valid options**, not
  choosing from a flat vocabulary

---

## Key Properties

1. **Deterministic verifiability** -- reasoning chains are provably valid
2. **Composability** -- link knowledge artifacts together like shared libraries
3. **Incremental learning** -- update a branch without retraining everything
4. **Interpretability for free** -- the structure IS the explanation
5. **I/O agnosticism** -- anything that can be verified works
6. **Deduplication** -- same concept hash = stored once

---

## Conceptual Example

```
"save this to the archive"

save -> persist -> write-to-storage -> [WASI boundary]
this -> current-context -> buffer-ref -> [memory/pure]
archive -> long-term-storage -> mount-point -> [WASI boundary]

If write-to-storage fails (disk full):
  remap: persist -> replicate -> network-storage -> succeeds
```

---

## The System Is

1. A graph of concept associations (B-tree stored, content-addressed)
2. An attention mechanism that picks which associations to follow given input
3. Execution by traversal -- walk the chain until it produces a result
4. Failure signals from reality (WASI syscalls) that trigger remapping
5. Remapping -- relink edges, rehash affected branches, try again

A **self-remapping concept graph that touches reality at its leaves.**

---

## Implementation Target

- **Language:** Rust
- **Form:** Library
- **Input:** NLP (natural language)
- **Output:** WASM/WASI instructions
- Pure concept resolution -> WASM instructions
- Resource/syscall operations -> WASI calls

---

## Workspace Structure

```
helix-dna/
  Cargo.toml                          # [workspace]
  crates/
    helix-dna-core/                   # concept network, B-tree storage, execution
      src/
        lib.rs
        concept.rs                    # ConceptId, Concept, ConceptKind
        store.rs                      # ContentAddressedStore (BTreeMap backed)
        network.rs                    # ConceptNetwork (directed graph + store)
        node.rs                       # Evaluate trait, ResultNode/ResourceNode
        execution.rs                  # Executor, pipeline runner, remapping
        error.rs
    helix-dna-nlp/                    # NLP input -> concept resolution
      src/
        lib.rs
        grammar.pest                  # PEG grammar
        tokenizer.rs
        parser.rs                     # structured intent extraction
        resolver.rs                   # parsed NL -> concept chain
        error.rs
    helix-dna-wasm/                   # concept chain -> WASM/WASI bytecode
      src/
        lib.rs
        codegen.rs                    # WASM module builder
        instructions.rs               # concept -> WASM instruction mapping
        wasi.rs                       # WASI import generation
        module.rs                     # module assembly
        error.rs
    helix-dna/                        # public facade crate
      src/
        lib.rs                        # compile(), HelixDna builder, re-exports
```

## Key Dependencies

- `blake3` -- content-addressing (fast, SIMD-accelerated hashing)
- `petgraph` -- directed graph for concept network (StableDiGraph for stable
  indices across remapping)
- `pest` / `pest_derive` -- PEG parser for NLP input
- `wasm-encoder` -- bytecodealliance WASM module builder
- `wasmtime` -- (dev) execute generated WASM in tests
- `wasmparser` -- (dev) validate generated WASM in tests
- `serde` -- serialization
- `thiserror` -- error types
- `proptest` -- (dev) property-based testing for state machine determinism

## Core Traits

```rust
/// Every concept implements this to participate in execution.
pub trait Evaluate {
    fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, NodeError>;
    fn is_pure(&self) -> bool;
}

/// Convert a concept chain into WASM instructions.
pub trait Compile {
    fn compile(&self, network: &ConceptNetwork, root: ConceptId) -> Result<Vec<u8>, CompileError>;
}

/// Resolve natural language into concept chains.
pub trait Resolve {
    fn resolve(&self, input: &str, network: &mut ConceptNetwork) -> Result<ConceptId, ResolveError>;
}
```

## MVP Goal

`compile("add 2 3")` produces valid WASM bytes that, when run, returns 5.
