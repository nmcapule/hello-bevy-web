#!/bin/bash

cargo build --target wasm32-unknown-unknown

wasm-bindgen \
    --out-name wasm_build \
    --out-dir web/target \
    --target web \
    target/wasm32-unknown-unknown/debug/hello-bevy-web.wasm

# python3 -m http.server --directory web 8080

# cargo install basic-http-server
basic-http-server -a 127.0.0.1:8080 web
