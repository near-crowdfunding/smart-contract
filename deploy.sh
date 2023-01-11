#!/bin/sh

./build.sh

echo ">> Deploying contract"

near dev-deploy --wasmFile ../target/wasm32-unknown-unknown/release/smart_contract.wasm --initFunction new --initArgs '{}' -f