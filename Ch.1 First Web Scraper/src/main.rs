use reqwest::Client;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .get("http://www.pythonscraping.com/pages/page1.html")
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&response);

    let h1_selector = Selector::parse("h1").unwrap();
    if let Some(h1_element) = document.select(&h1_selector).next() {
        println!("{}", h1_element.text().collect::<String>());
    } else {
        println!("No <h1> element found.");
    }

    Ok(())
}
