# hello-wasm

Rust + WebAssembly の最小サンプルです。

## 前提

- `rustup` / `cargo` / `wasm-pack` がインストール済み
- `wasm32-unknown-unknown` ターゲットが追加済み

```bash
rustup target add wasm32-unknown-unknown
```

## ビルド手順

`hello-wasm` ディレクトリで実行します。

```bash
cd /home/yuki/repos/hello-rust/202602wasm/ch02/hello-wasm
wasm-pack build --target web
```

ビルドに成功すると、`pkg/` に Wasm と JS ラッパーが生成されます。

## 実行手順（ブラウザ）

ローカル HTTP サーバを起動します。

```bash
cd /home/yuki/repos/hello-rust/202602wasm/ch02/hello-wasm
python3 -m http.server 8000
```

ブラウザで以下を開きます。

- `http://localhost:8000`

`greet() を実行` ボタンを押すと、Rust 側の関数結果が画面に表示されます。

## 実行手順（npm）

`npm` だけでローカルサーバを起動する場合は次を実行します。

```bash
cd /home/yuki/repos/hello-rust/202602wasm/ch02/hello-wasm
npm exec --yes serve . -l 8000
```

ブラウザで以下を開きます。

- `http://localhost:8000`

初回は `serve` パッケージの取得が走るため、ネットワーク接続が必要です。

## ファイル構成（主要）

- `src/lib.rs`: `#[wasm_bindgen]` 付きの Rust 関数
- `pkg/`: `wasm-pack` が生成する成果物
- `main.js`: Wasm 初期化と `greet()` 呼び出し
- `index.html`: ブラウザ表示用のページ
