cargo build --target wasm32-unknown-unknown;
cp target/wasm32-unknown-unknown/debug/circuitsim.wasm .;
python -m http.server 8000;
