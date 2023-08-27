use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonscraping.com/pages/warandpeace.html";
    let html = fetch_html(url).await?;

    let headings = find_all_tags(&html, "h1, h2, h3, h4, h5, h6");
    for heading in headings {
        println!("{}", heading);
    }

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn find_all_tags(html: &str, tags: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(tags).unwrap();

    let tags: Vec<String> = document
        .select(&selector)
        .map(|element| element.text().collect())
        .collect();

    tags
}
