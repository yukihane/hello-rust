## How to execute

```bash
wasm-pack build --target web
cargo install miniserve
miniserve . --index "index.html" -p 8080
```