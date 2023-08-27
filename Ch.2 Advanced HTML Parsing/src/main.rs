pub mod html_tree_printer;

use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonscraping.com/pages/page3.html";
    let html = fetch_html(url).await?;

    let table_id = "giftList";
    let children = iterate_table_children(&html, table_id);
    for child in children {
        println!("{}", child);
    }

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn iterate_table_children(html: &str, table_id: &str) -> Vec<String> {
    let document = Html::parse_document(html);

    let selector = Selector::parse(&format!("#{} tr", table_id)).unwrap();

    let children: Vec<String> = document
        .select(&selector)
        .map(|element| element.text().collect())
        .collect();

    children
}
