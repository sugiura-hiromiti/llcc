# llcc overview
- Multi-crate Rust workspace with crates: core, error, proc_macro, semantics, orchestrator. Purpose not explicitly documented; crates suggest language/compiler components (asm, parser, tokenizer, semantics, orchestrator) with shared error handling. No README or docs found.
- Tooling: Nix flake for dev shell (`flake.nix`) that installs latest Rust toolchain via fenix and prints tool locations on shell entry.
- Entry points: No binaries observed; crates expose libraries. Orchestrator crate may coordinate components.
- Config: .rustfmt.toml, .dprint.json, .editorconfig present.
- Source structure: Each crate under `crates/<name>/src`. Core crate includes modules like asm/front/parser/register/semantic_core/tokenizer. Orchestrator has lib.rs and file_manage.rs. Others not inspected yet.