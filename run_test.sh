set -e 
echo "Compiling wasm_blob"
cargo build -p wasm_blob --target wasm32-wasi --release
echo "Optimising wasm_blob"
wasm-opt target/wasm32-wasi/release/wasm_blob.wasm -o wasm_blob.wasm -O2 --remove-imports
echo "Compiling app"
cargo build -p app --release
echo "Running app"
./target/release/app