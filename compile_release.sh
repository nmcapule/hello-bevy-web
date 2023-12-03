#!/bin/bash

cargo build --target wasm32-unknown-unknown --release

wasm-bindgen \
    --out-name wasm_build \
    --out-dir web/target \
    --target web \
    target/wasm32-unknown-unknown/release/hello-bevy-web.wasm

wasm-opt \
    -Os \
    -o web/target/wasm_build_bg_optimized.wasm \
    web/target/wasm_build_bg.wasm

tar \
    -cvf web/target/wasm_build_bg_optimized.wasm.tar.br \
    --use-compress-program="brotli -Z" \
    web/target/wasm_build_bg_optimized.wasm