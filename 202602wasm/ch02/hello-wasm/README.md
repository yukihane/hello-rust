# hello-wasm 学習ロードマップ

このディレクトリは Rust + WebAssembly 学習用の最小プロジェクトです。

## 1. 学習ロードマップ（初心者向け）

### Week 1: 最小構成を動かす
- `wasm-pack build --target web` を通す
- `web/index.html` から `greet()` を呼ぶ
- 「初期化は非同期、関数呼び出しは同期」を理解する

### Week 2: Rust/JS 間のデータ受け渡し
- `String`, `number`, `bool` の往復
- `Result<T, E>` と `JsValue` の扱い
- `#[wasm_bindgen]` 公開 API の設計に慣れる

### Week 3: ブラウザ API と非同期
- `web-sys` で DOM 操作
- `fetch` 連携（`wasm-bindgen-futures`）
- UI イベントと Rust 関数を結びつける

### Week 4: テストとデバッグ
- `wasm-bindgen-test` で最小テスト
- エラーログと panic の見方を確認
- 変更後にビルドと手動確認を繰り返す

### Week 5: 最適化と配布
- release ビルドのサイズ確認
- `wasm-opt` を使った最適化
- `pkg/` の役割（配布成果物）を整理

## 2. このプロジェクトのディレクトリ構成（見直し後）

実装コードとブラウザ側コードを分離しました。

```text
hello-wasm/
  Cargo.toml
  src/
    lib.rs            # Rust の公開 API（wasm 側）
  web/
    index.html        # ブラウザ表示
    main.js           # wasm 初期化と関数呼び出し
  pkg/                # wasm-pack build で生成される成果物
  target/             # Rust ビルド生成物（git 管理外）
```

## 3. セットアップ

```bash
rustup target add wasm32-unknown-unknown
```

## 4. ビルド

```bash
cd /home/yuki/repos/hello-rust/202602wasm/ch02/hello-wasm
wasm-pack build --target web
```

## 5. 実行（ブラウザ）

```bash
cd /home/yuki/repos/hello-rust/202602wasm/ch02/hello-wasm
python3 -m http.server 8000
```

ブラウザで `http://localhost:8000/web/` を開きます。

## 6. 実行（npm）

```bash
cd /home/yuki/repos/hello-rust/202602wasm/ch02/hello-wasm
npm exec --yes serve . -l 8000
```

ブラウザで `http://localhost:8000/web/` を開きます。

## 7. 補足

- `pkg/` の中身は `src/lib.rs` 単独ではなく、`Cargo.toml` の情報と `wasm-bindgen` の生成コードを含んで作られます。
- `pkg/` は再生成前提なので手編集しません。
