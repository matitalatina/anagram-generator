# Anagram Generator

## Algorithm

- You can find it in `algorithm` folder.
- Written in Rust.
- Build in WASM with `wasm-pack build`.
- You can build it also to use it in shell.

### Requirements

- [rustup](https://rustup.rs/).
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) if you want to build WASM version.

### Getting started

### CLI version

- Go to `algorithm` folder.
- `cargo build --release` to build the binary. You can find it in `target/release/anagram-generator`.
- Run it and read the helper to learn how to use it.
- You can directly run with `cargo run --release -- <CUSTOM_PARAMETERS...>`

### WASM version

- Go to `algorithm` folder.
- Build in WASM with `wasm-pack build`.
- You can find its node package in `pkg`.

## Frontend

- You can find it `frontend` folder.
- Written in TypeScript + React.

### Requirements

- [Node.js](https://nodejs.org/it/) `>= 10`.

### Getting Started 

- Go to `frontend` folder.
- `npm install`.
- `npm run start` to see it live locally.


# Credits

- Italian dictionary taken from https://github.com/napolux/paroleitaliane
