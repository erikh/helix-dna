# Helix-DNA Migration Plan

## Completed

1. Renamed all crate directories (`lassie-*` → `helix-dna-*`)
2. Updated all Cargo.toml files (package names, dependency names, paths)
3. Updated all Rust source files (`lassie_core` → `helix_dna_core`, `Lassie` → `HelixDna`, `LassieError` → `HelixDnaError`, etc.)
4. Updated DESIGN.md with all references changed
5. Changed license from MIT to AGPL-3.0-only + created LICENSE file
6. Build, tests (33/33), and clippy all pass
7. `git init` was run (in old path)

## Remaining

1. Rename parent directory from `lassie/` to `helix-dna/` (if not already done)
2. Fix shell CWD to point at new directory
3. Create GitHub repo `erikh/helix-dna` (public)
4. Commit all files and push to main
5. Publish crates to crates.io in dependency order:
   - `helix-dna-core` (no internal deps — publish first)
   - `helix-dna-nlp` (depends on core)
   - `helix-dna-wasm` (depends on core)
   - `helix-dna` (depends on all three — publish last)
