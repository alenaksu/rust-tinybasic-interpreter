rm -rf out
rm -rf target
cargo build --target=wasm32-unknown-unknown --release --lib
wasm-bindgen --out-dir out --target web target/wasm32-unknown-unknown/release/tinybasic.wasm
