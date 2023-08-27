use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();

    let url = "http://pythonscraping.com/pages/page1.html";

    match client.get(url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(body) => {
                        println!("Body: {}", body);
                    }
                    Err(err) => {
                        eprintln!("Error reading response body: {}", err);
                    }
                }
            } else {
                eprintln!("Request was not successful: {:?}", response.status());
            }
        }
        Err(err) => {
            eprintln!("Error sending HTTP request: {}", err);
        }
    };
}
