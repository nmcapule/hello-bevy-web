#!/bin/bash

cargo build --target wasm32-unknown-unknown

wasm-bindgen \
    --out-name wasm_build \
    --out-dir web/target \
    --target web \
    target/wasm32-unknown-unknown/debug/hello-bevy-web.wasm

python3 -m http.server --directory web
