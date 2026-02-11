# 202602wasm 学習ロードマップ

Rust + WebAssembly をこれから学ぶためのワークスペースです。

## 1. 学習ロードマップ（8週間）

### Week 1: 最小構成を動かす
- `wasm-pack build --target web` を通す
- `await init()` と `greet()` 呼び出しまで確認する

### Week 2: Rust/JS 相互運用の基礎
- 文字列・数値・真偽値の受け渡し
- `Result` / `JsValue` の扱い

### Week 3: ブラウザ API 連携
- `web-sys` で DOM 操作
- `fetch` など非同期処理の連携

### Week 4: テストとデバッグ
- `wasm-bindgen-test` の導入
- エラーハンドリングとログ確認

### Week 5: サイズ最適化
- release ビルドとサイズ計測
- `wasm-opt` による最適化

### Week 6: フレームワーク入門
- Leptos / Yew / Dioxus のどれか1つで小規模アプリ作成

### Week 7: Component Model 入門
- WIT の基本と component の実行

### Week 8: 小さな実践プロジェクト
- テストあり・配布可能な状態まで仕上げる

## 2. このリポジトリでの進め方

- Week 1 は `week01/hello-wasm` を使ってビルドと実行を確認
- Week 2 以降は `week02`〜`week08` に演習プロジェクトを追加して進める
- 各週の狙いは各ディレクトリの `README.md` に記載

## 3. 現在の構成

```text
202602wasm/
  README.md
  week01/
    hello-wasm/
      Cargo.toml
      src/
      web/
      pkg/
  week02/
    README.md
  week03/
    README.md
  week04/
    README.md
  week05/
    README.md
  week06/
    README.md
  week07/
    README.md
  week08/
    README.md
```

## 4. 参考（補助）

- wasm-bindgen Guide: https://wasm-bindgen.github.io/wasm-bindgen/
- wasm-pack book（補助扱い）: https://drager.github.io/wasm-pack/book/introduction.html
