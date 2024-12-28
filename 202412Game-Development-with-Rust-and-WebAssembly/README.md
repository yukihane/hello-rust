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
