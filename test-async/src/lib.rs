// use reqwest::Error;

async fn my_func() -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    println!("body = {body:?}");

    Ok("OK".to_string())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn it_works() {
        let result = my_func().await.unwrap();
        assert_eq!(result, "OK");
    }
}
