[RustとWebAssemblyによるゲーム開発](../202412Game-Development-with-Rust-and-WebAssembly/) を始めようと思ったのだが、webpack使ったりとかどうも現在の標準的な環境と比べて古臭いように感じたので、どうあるべきかを検討します。

同じような懸念を表明しているトピックがあった(2023-08):

- [For Rust+WASM, what alternatives are there to Webpack?](https://users.rust-lang.org/t/98711)

これによると:

- [bundlerを利用せずに環境を作る](https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html)
- [ViteのWebAssemblyサポート](https://ja.vite.dev/guide/features.html#webassembly)
- [手動でwasmをロードするサンプル](https://github.com/kettle11/hello_triangle_wasm_rust/)
- Rollupのプラグイン[rollup-plugin-rust](https://github.com/wasm-tool/rollup-plugin-rust)

"rustlang wasm vite" 辺りの単語で検索するとセットアップ方法はたくさん出てくる。
が、おそらくbundlerは無くても良いと思うので最初の手順でセットアップすることにしてみる。
