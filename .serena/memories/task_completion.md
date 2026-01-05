# Task completion checklist
- Run `cargo fmt` to ensure Rust code matches formatting rules (.rustfmt.toml).
- If non-Rust files changed, run `dprint fmt` for JSON/Markdown/TOML/YAML/TS as applicable.
- Run `cargo test` (and optionally `cargo clippy --all-targets --all-features`) to verify workspace builds/tests pass.
- Ensure changes are staged (`git status`, `git add`) and provide summary in commit message.
- If using Nix, consider running commands inside `nix develop` environment.