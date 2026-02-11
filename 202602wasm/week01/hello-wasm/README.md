# Week 1: hello-wasm

Week 1 の目標は、Rust + WebAssembly の最小構成をビルドしてブラウザで実行することです。

## 1. 到達目標

- `wasm-pack build --target web` を成功させる
- `web/index.html` から `greet()` を呼び出して表示する
- 「初期化は非同期、関数呼び出しは同期」を確認する

## 2. ディレクトリ構成

```text
hello-wasm/
  Cargo.toml
  src/lib.rs          # Rust の公開 API（wasm 側）
  web/index.html      # ブラウザ表示
  web/main.js         # wasm 初期化と関数呼び出し
  pkg/                # wasm-pack 生成物
  target/             # Rust ビルド生成物（git 管理外）
```

## 3. セットアップ

```bash
rustup target add wasm32-unknown-unknown
```

## 4. ビルド

```bash
cd /home/yuki/repos/hello-rust/202602wasm/week01/hello-wasm
wasm-pack build --target web
```

## 5. 実行（ブラウザ）

```bash
cd /home/yuki/repos/hello-rust/202602wasm/week01/hello-wasm
python3 -m http.server 8000
```

ブラウザで `http://localhost:8000/web/` を開きます。

## 6. 実行（npm）

```bash
cd /home/yuki/repos/hello-rust/202602wasm/week01/hello-wasm
npm exec --yes serve . -l 8000
```

ブラウザで `http://localhost:8000/web/` を開きます。

## 7. 確認ポイント

- `await init()` が初期化（非同期）
- `greet()` 呼び出し自体は同期
- `pkg/` は再生成前提のため手編集しない
