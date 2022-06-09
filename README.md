# duid
Djedou UI Design in Rust 


# Add the WebAssembly target
rustup target add wasm32-unknown-unknown
# Build the app
cargo build --release --target=wasm32-unknown-unknown
wasm-gc target/wasm32-unknown-unknown/release/duid.wasm

wasm-pack build 

basic-http-server -a 127.0.0.1:4000