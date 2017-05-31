#!/bin/bash

cargo build --release --target=wasm32-unknown-emscripten
cp target/wasm32-unknown-emscripten/release/deps/* target/wasm32-unknown-emscripten/release/

echo ""
echo "open http://localhost:8080/out.html"
echo ""

python2 -m SimpleHTTPServer 8080
