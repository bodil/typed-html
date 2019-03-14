#!/bin/bash

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen ../../target/wasm32-unknown-unknown/release/typed_html_web_sys_test.wasm --no-modules --out-dir ../../target/wasm32-unknown-unknown/release
