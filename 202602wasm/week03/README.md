# Week 3: ブラウザ API 連携

## 目標
- `web-sys` で DOM 操作を行う
- 非同期 API（`fetch`）の結果を画面表示する

## 演習案
- ボタン押下で API を呼び、結果を画面に表示する

## 進め方（具体ステップ）
1. Week2 のプロジェクトをコピーして作業用にする
   - 例: `week03/hello-wasm` を作成して `week01/hello-wasm` をベースにする
2. `web-sys` と `wasm-bindgen-futures` を `Cargo.toml` に追加する
3. Rust 側に非同期で値を返す関数を追加する
   - 最初は外部APIではなく、遅延付きのダミー結果でもよい
4. JS 側でボタンクリック時に `await` して結果を表示する
5. 余力があれば `fetch` を使って公開APIからJSONを取得する

## 動作確認
- `wasm-pack build --target web`
- `python3 -m http.server 8000`
- `http://localhost:8000/web/` でボタン押下時に結果が更新されること

## 完了条件
- Rust で非同期関数を1つ公開できている
- JS から `await` で呼び出して画面更新できている
