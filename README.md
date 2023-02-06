# Rust-WASM-Svelte-Playground
Playground and showcase for WASM (from Rust code) + Svelte web app interaction

(Based on this [tutorial](https://blog.logrocket.com/integrating-svelte-app-rust-webassembly/))

## ... in a nutshell
- [Svelte in a nutshell](Svelte%20in%20a%20nutshell.md)
- [WASM in a nutshell](WASM%20in%20a%20nutshell.md)

**TLDR:** [A presentation can be found here](./Presentation/Presentation.pdf)
(HTML version also available; source [here](./Presentation/Presentation.md) based on [Marp](https://marp.app/) markdown)

## Code
- [Rust](rust%2FREADME.md)
- [Svelte](svelte%2FREADME.md)

## Interaction
Works via [Rust WASM Bindgen](https://rustwasm.github.io/wasm-bindgen/)
which can be picked up from JS as seen [here in main.js](svelte%2Fsrc%2Fmain.js)

**Nice:** Hot reload works for Rust code out-of-the-box (via WASM Rollup plugin) - just keep the Svelte app running. 