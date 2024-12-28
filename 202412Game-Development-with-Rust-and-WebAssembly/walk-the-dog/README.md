## How to execute

```bash
wasm-pack build --target web
cargo install miniserve
miniserve . --index "index.html" -p 8080
```

## Download asstes

p.29, 2.3 Canvasへのスプライトの描画

```bash
curl -L -O https://github.com/PacktPublishing/Game-Development-with-Rust-and-WebAssembly/wiki/walk_the_dog_assets-0.0.7.zip
unzip walk_the_dog_assets-0.0.7.zip
```
