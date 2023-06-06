cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./dist/web --target web ./target/wasm32-unknown-unknown/release/starstorm.wasm