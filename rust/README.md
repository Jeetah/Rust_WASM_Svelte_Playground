# Resources
- Rust WASM: https://rustwasm.github.io

## Why Rust + WASM
- Rust recommends WASM when coding for the web: [https://www.rust-lang.org/what/wasm]([https://www.rust-lang.org/what/wasm)
  - [Deep Dive](https://rustwasm.github.io/docs/book/)
  - Rust offers [access to Web APIs](https://docs.rs/web-sys/0.3.60/web_sys/) (and thus DOM access)

# Tooling
- Compiles Rust to WASM with help of [wasm-pack](https://rustwasm.github.io/wasm-pack/)
- Build: _cargo build_
- Build for Web: _wasm-pack build --target web_ --dev

## Check generated code
...like the results of something like this #[derive(Clone)]
- Cargo expand: https://github.com/dtolnay/cargo-expand (needs nightly toolchain installation)

## Raise WASM stack memory
Normally 1 MB, raise -> see config in _.cargo_ directory