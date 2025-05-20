use hello_google_oauth2::oauth;

#[tokio::main]
pub async fn main() {
    let _ = oauth().await;
}
