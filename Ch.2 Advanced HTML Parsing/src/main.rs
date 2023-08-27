use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonscraping.com/pages/warandpeace.html";
    let html = fetch_html(url).await?;

    let elements = find_elements_by_id(&html, "text");
    for element in elements {
        println!("{}", element);
    }

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn find_elements_by_id(html: &str, id_value: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(&format!("#{}", id_value)).unwrap();

    let elements: Vec<String> = document
        .select(&selector)
        .map(|element| element.text().collect())
        .collect();

    elements
}
