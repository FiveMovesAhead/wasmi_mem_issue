set -e 
echo "Compiling wasm_blob"
cargo build -p wasm_blob --target wasm32-wasi --release
echo "Optimising wasm_blob"
wasm-opt target/wasm32-wasi/release/wasm_blob.wasm -o wasm_blob.wasm -O2 --remove-imports
echo "Compiling wasmi_app"
cargo build -p wasmi_app --release
echo "Running wasmi_app"
./target/release/wasmi_app || true
echo "Compiling wasmtime_app"
cargo build -p wasmtime_app --release
echo "Running wasmtime_app"
./target/release/wasmtime_app || true