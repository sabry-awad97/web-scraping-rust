use std::error::Error;

use reqwest::{redirect, Client, Response};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://github.com";
    let response = fetch_url_with_redirect_handling(url).await?;

    println!("Final URL after following redirects: {}", response.url());
    println!("Response status: {}", response.status());

    Ok(())
}

async fn fetch_url_with_redirect_handling(url: &str) -> Result<Response, reqwest::Error> {
    let client = Client::builder()
        .redirect(redirect::Policy::limited(5))
        .build()?;

    let response = client.get(url).send().await?;
    Ok(response)
}
