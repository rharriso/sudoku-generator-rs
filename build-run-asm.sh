#!/bin/bash

cargo build --release --target=asmjs-unknown-emscripten

echo ""
echo "open http://localhost:8080/out.asm.html"
echo ""

python2 -m SimpleHTTPServer 8080
