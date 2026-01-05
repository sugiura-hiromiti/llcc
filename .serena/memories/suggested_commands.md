# Suggested commands
- Enter dev shell with Nix (if using flakes): `nix develop` or `nix develop .` to get Rust toolchain (from fenix) with shell hook showing tool paths.
- Build: `cargo build` (workspace enabled).
- Format Rust: `cargo fmt` (uses .rustfmt.toml).
- Lint/clippy: `cargo clippy --all-targets --all-features` (not configured explicitly but standard).
- Test: `cargo test` (workspace tests).
- Format non-Rust (TS/JSON/MD/TOML/YAML): `dprint fmt` if dprint available (config at .dprint.json).
- Check workspace members: `cargo metadata --no-deps`.
- Git helpers: standard `git status`, `git diff`, `git add`, `git commit`.