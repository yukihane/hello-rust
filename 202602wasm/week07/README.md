# Week 7: Component Model 入門

## 目標
- WIT の基本を理解する
- component の実行フローを確認する

## 演習案
- 最小 component を1つ作って実行する

## 進め方（具体ステップ）
1. WIT ファイルを1つ作る
   - 入力1つ、出力1つの最小インターフェース
2. Rust 側で WIT に対応する実装を書く
3. component をビルドする
4. ランタイム（例: wasmtime）で呼び出して結果確認する
5. 通常の `wasm-bindgen` 方式との違いをメモする

## 動作確認
- component 実行コマンドで期待値が返る

## 完了条件
- WIT と Rust 実装の対応を説明できる
- 実行手順が README に残っている
