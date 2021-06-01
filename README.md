### Requirements
- nightly rust
```
rustup update nightly
```
- wasi support
```
rustup target add wasm32-wasi --toolchain nightly
```
- [wasmer](https://github.com/wasmerio/wasmer), wasm runtime environment.(or [wasmtime](https://github.com/bytecodealliance/wasmtime) which is using by substrate)
```
curl https://get.wasmer.io -sSfL | sh
```

### Compile
```
cargo +nightly build --target=wasm32-wasi --release
```

### Run
```
wasmer run --enable-simd ./target/wasm32-wasi/release/simd-in-wasm.wasm
```
