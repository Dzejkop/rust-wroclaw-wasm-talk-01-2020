cargo build --target wasm32-unknown-unknown --release --no-default-features --features python_wasmtime
cp target/wasm32-unknown-unknown/release/wasm_conway.wasm python/wasm_conway.wasm
