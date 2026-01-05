# Style and conventions
- Rust formatting controlled by `.rustfmt.toml`: uses tabs (`hard_tabs = true`), max width 80, wrap comments, always trailing commas, imports granularity Item, reorder_impl_items true, style edition 2024, various unstable formatting features enabled. Respect automated `cargo fmt` with this config.
- `.editorconfig` present (not inspected yet) and `.dprint.json` sets formatting rules for TypeScript/JSON/Markdown/TOML/YAML (tabs for most; YAML uses spaces). Use dprint where relevant.
- No explicit naming or docstring conventions documented beyond standard Rust practices.