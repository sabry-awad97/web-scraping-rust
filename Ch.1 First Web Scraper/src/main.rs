use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .get("http://pythonscraping.com/pages/page1.html")
        .send()
        .await?;

    if response.status().is_success() {
        // Read the response body as a string
        let body = response.text().await?;

        println!("Body: {}", body);
    } else {
        println!("Request was not successful: {:?}", response.status());
    }

    Ok(())
}
