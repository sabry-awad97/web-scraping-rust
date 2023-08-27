use scraper::Html;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonscraping.com/pages/warandpeace.html";
    let html = fetch_html(url).await?;

    let target_text = "the prince";
    let count = count_text_occurrences(&html, target_text);

    println!("Number of occurrences: {}", count);

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn count_text_occurrences(html: &str, target_text: &str) -> usize {
    let document = Html::parse_document(html);
    let count = document
        .root_element()
        .text()
        .filter(|text| text.contains(target_text))
        .count();

    count
}
