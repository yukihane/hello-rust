use google_drive3::{DriveHub, Error, hyper_rustls, hyper_util, yup_oauth2};

pub async fn oauth() {
    // Get an ApplicationSecret instance by some means. It contains the `client_id` and
    // `client_secret`, among other things.
    let secret: yup_oauth2::ApplicationSecret = Default::default();
    // Instantiate the authenticator. It will choose a suitable authentication flow for you,
    // unless you replace  `None` with the desired Flow.
    // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
    // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
    // retrieve them from storage.
    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await
    .unwrap();

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_or_http()
                .enable_http1()
                .build(),
        );
    let mut hub = DriveHub::new(client, auth);
    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `doit()`.
    // Values shown here are possibly random and not representative !
    let result = hub
        .files()
        .list()
        .team_drive_id("et")
        .supports_team_drives(true)
        .supports_all_drives(false)
        .spaces("amet.")
        .q("takimata")
        .page_token("amet.")
        .page_size(-20)
        .order_by("ipsum")
        .include_team_drive_items(true)
        .include_permissions_for_view("Lorem")
        .include_labels("gubergren")
        .include_items_from_all_drives(false)
        .drive_id("dolor")
        .corpus("ea")
        .corpora("ipsum")
        .doit()
        .await;

    match result {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
            Error::HttpError(_)
            | Error::Io(_)
            | Error::MissingAPIKey
            | Error::MissingToken(_)
            | Error::Cancelled
            | Error::UploadSizeLimitExceeded(_, _)
            | Error::Failure(_)
            | Error::BadRequest(_)
            | Error::FieldClash(_)
            | Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(res) => println!("Success: {:?}", res),
    }
}
