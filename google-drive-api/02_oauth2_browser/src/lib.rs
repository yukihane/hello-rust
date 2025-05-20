use std::pin::Pin;

use google_drive3::yup_oauth2::{
    self, InstalledFlowAuthenticator, InstalledFlowReturnMethod,
    authenticator_delegate::InstalledFlowDelegate,
};

struct MyFlowDelegate;

impl InstalledFlowDelegate for MyFlowDelegate {
    fn present_user_url<'a>(
        &'a self,
        url: &'a str,
        _need_code: bool,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + 'a>> {
        Box::pin(async move {
            println!("url: {}", url);
            open::that(url).map_err(|e| format!("{:?}", e))?;
            Ok("".to_string())
        })
    }
}
pub async fn oauth() {
    // Read application secret from a file. Sometimes it's easier to compile it directly into
    // the binary. The clientsecret file contains JSON like `{"installed":{"client_id": ... }}`
    let secret = yup_oauth2::read_application_secret("clientsecret.json")
        .await
        .expect("clientsecret.json");

    // Create an authenticator that uses an InstalledFlow to authenticate. The
    // authentication tokens are persisted to a file named tokencache.json. The
    // authenticator takes care of caching tokens to disk and refreshing tokens once
    // they've expired.
    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .flow_delegate(Box::new(MyFlowDelegate))
        .persist_tokens_to_disk("tokencache.json")
        .build()
        .await
        .unwrap();

    let scopes = &["https://www.googleapis.com/auth/drive.file"];

    // token(<scopes>) is the one important function of this crate; it does everything to
    // obtain a token that can be sent e.g. as Bearer token.
    match auth.token(scopes).await {
        Ok(token) => println!("The token is {:?}", token),
        Err(e) => println!("error: {:?}", e),
    }

    println!("end of oauth");
}
