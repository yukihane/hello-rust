# RustとWebAssemblyによるゲーム開発

## 基本情報

- 書籍情報: https://www.oreilly.co.jp/books/9784814400393/
- コード: https://github.com/PacktPublishing/Game-Development-with-Rust-and-WebAssembly
- [templateのコード](https://github.com/rustwasm/rust-webpack-template/tree/master/template)
  - 書籍の最初の方ではこのコードをもとに変更している

## 誤植(正誤表にあるもの以外)

### p.10 1.4 HTML Canvasを用いたスクリーンへの描画

誤

> このコメントと[cfg(debug_annotations)]アノテーションを削除しよう

正

> このコメントと[cfg(debug_assersions)]アノテーションを削除しよう

## NOTE

### npm init rust-webpack が動かない

1.3 RustとWebAssemblyプロジェクトのスケルトンの作成。

```bash
npm init rust-webpack
```

を実行すると

```
npm error could not determine executable to run
npm error A complete log of this run can be found in: /Users/xxxx/.npm/_logs/2024-12-05T04_56_06_425Z-debug-0.log
```

というエラーになる。

- [npm init --verbose rust-webpack my-app fails with Error: could not determine executable to run #190](https://github.com/rustwasm/rust-webpack-template/issues/190)

にワークアラウンドの記述あり。
しかしこのパッケージ `rust-webpack-template` は最新リリースが2019年であり、もはや利用すべきでは無いのかも？

他方、 [Rust 🦀 and WebAssembly 🕸](https://rustwasm.github.io/docs/book/) の方法は:

```bash
cargo install wasm-pack
cargo install cargo-generate
npm install npm@latest -g
```

```bash
cargo generate --git https://github.com/rustwasm/wasm-pack-template
```

プロジェクト名は `walk-the-dog` を指定。

```bash
cd walk-the-dog
npm init wasm-app www
rm -rf www/.git
cd www
npm install
```

この問題については結局bunlderを使わない方法で解消した。
- ref. https://github.com/yukihane/hello-rust/tree/main/202412hello-wasm

### p.25 1.4.3 context.set_fill_style() が deprecated

```
     let color_str = format!("rgb({},{},{})", color.0, color.1, color.2);
-    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&color_str));
+    context.set_fill_style_str(&color_str);
```

### p.33 この時点で画像がロードできる

> まだ画像は表示されない。まだ実際に画像がロードされるのを待つようになっていないからだ。
それを行うには非同期関数が必要になる。

とあるが、キャッシュが効いていると？画像は想定に反して表示されるようだ。

### p.45-46 wasm-bindgen into_serde の deprecated 対応

> warning: use of deprecated method `wasm_bindgen::JsValue::into_serde`: causes dependency cycles, use `serde-wasm-bindgen` or `gloo_utils::format::JsValueSerdeExt` instead

という警告メッセージが出るので [`serde-wasm-bindgen`](https://docs.rs/serde-wasm-bindgen/latest/serde_wasm_bindgen/) を利用する。

```diff
diff --git a/202412Game-Development-with-Rust-and-WebAssembly/walk-the-dog/Cargo.toml b/202412Game-Development-with-Rust-and-WebAssembly/walk-the-dog/Cargo.toml
index 5c155b6..f5a90d3 100644
--- a/202412Game-Development-with-Rust-and-WebAssembly/walk-the-dog/Cargo.toml
+++ b/202412Game-Development-with-Rust-and-WebAssembly/walk-the-dog/Cargo.toml
@@ -12,6 +12,7 @@ futures = "0.3.31"
 getrandom = { version = "0.2.15", features = ["js"] }
 rand = "0.8.5"
 serde = { version = "1.0.217", features = ["derive"] }
+serde-wasm-bindgen = "0.6.5"
 wasm-bindgen = { version = "0.2.99", features = ["serde-serialize"] }
 wasm-bindgen-futures = "0.4.49"
 web-sys = { version = "0.3.76", features = ["CanvasRenderingContext2d", "Document", "Element", "HtmlCanvasElement", "HtmlImageElement", "Response", "Window", "console"] }
```

```diff
diff --git a/202412Game-Development-with-Rust-and-WebAssembly/walk-the-dog/src/lib.rs b/202412Game-Development-with-Rust-and-WebAssembly/walk-the-dog/src/lib.rs
index f3f0d5b..af3d01e 100644
--- a/202412Game-Development-with-Rust-and-WebAssembly/walk-the-dog/src/lib.rs
+++ b/202412Game-Development-with-Rust-and-WebAssembly/walk-the-dog/src/lib.rs
@@ -152,8 +152,7 @@ pub fn main_js() -> Result<(), JsValue> {
             .await
             .expect("Could not fetch rhb.json");

-        let sheet: Sheet = json
-            .into_serde()
+        let sheet: Sheet = serde_wasm_bindgen::from_value(json)
             .expect("Could not convert rhb.json into a Sheet structure");
     });
```
