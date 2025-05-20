## やりたいこと

[google-drive3](https://docs.rs/google-drive3/latest/google_drive3/) で google drive api を操作したい。
それに先立って、oauth2.0 で認可を得る実装を確認したい。

google-drive3 の内部では [yup-oauth2](https://docs.rs/yup-oauth2/latest/yup_oauth2/) が利用されているようなので、こちらのドキュメントを見ればよいのではないか。

## できたこと

google-drive3 が reexport しているもの

```
use google_drive3::yup_oauth2::{self, InstalledFlowAuthenticator, InstalledFlowReturnMethod};
```

を利用して、yup-oauth2 のサンプルコードを動かし、oauth2.0 で認可を得る。

## 実行方法

1. `clientsecret.json` を入手し、プロジェクトルートに置く
   - [Rust で GoogleDrive を操作する - Zenn](https://zenn.dev/nodamushi/articles/384e104b6497f2) の "Google Cloude から client_secret.json を取得するまで" 節に説明がある
1. `cargo run` 実行
1. 標準出力に、URL が表示されるのでそれを Web ブラウザーで開き、認可する
1. `tokencache.json` ファイルが生成される
1. (再度 `cargo run` を実行すると認可フローがスキップされ、 `tokencache.json` が利用されているのがわかる)
