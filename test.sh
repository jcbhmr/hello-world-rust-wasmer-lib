#!/bin/bash
set -e
cargo wasmer --dry-run --debug --out-dir=./out
mv ./out/wasmer.toml ./out/wapm.toml

wasmer-pack js ./out --out-dir=./tests/js
(cd tests/js && npm install && npm test)

# wasmer-pack py ./out --out-dir=./tests/py/package
