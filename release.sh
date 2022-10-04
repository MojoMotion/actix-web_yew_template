#!/bin/bash
set -eu

cd front-end
cargo build --release --target wasm32-unknown-unknown

wasm-bindgen target/wasm32-unknown-unknown/release/front-end.wasm \
--out-dir ../dist --no-modules --no-typescript
