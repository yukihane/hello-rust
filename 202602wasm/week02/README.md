# Week 2: Rust/JS 相互運用の基礎

## 目標
- 文字列・数値・真偽値の受け渡し
- `Result` / `JsValue` の扱い

## 演習案
- `week01/hello-wasm` を複製し、引数あり関数を3つ追加する
- エラーを返す関数を1つ作り、JS側で表示する

## 進め方（具体ステップ）
1. `greet()` を引数ありに変更する
   - `pub fn greet(name: &str) -> String` にして、JS から文字列を渡す
2. 数値・真偽値を扱う関数を追加する
   - 例: `sum(a: i32, b: i32) -> i32`
   - 例: `is_even(n: i32) -> bool`
3. エラーを返す関数を1つ追加する
   - 例: 空文字なら `Err(JsValue::from_str("name is empty"))`
   - JS 側で `try/catch` して表示する
4. 変更ごとにビルドと実行で確認する
   - `wasm-pack build --target web`
   - `python3 -m http.server 8000`
   - `http://localhost:8000/web/` にアクセス
