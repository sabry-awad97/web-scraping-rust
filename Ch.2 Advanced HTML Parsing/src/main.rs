use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonscraping.com/pages/warandpeace.html";
    let html = fetch_html(url).await?;

    let class_name = "green";
    if let Some(names) = get_elements_with_class(&html, class_name) {
        for name in names {
            println!("{}", name);
        }
    } else {
        println!("No names found.");
    }

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn get_elements_with_class(html: &str, class_name: &str) -> Option<Vec<String>> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(&format!(".{}", class_name)).unwrap();

    let names: Vec<String> = document
        .select(&selector)
        .map(|element| element.text().collect())
        .collect();

    if names.is_empty() {
        None
    } else {
        Some(names)
    }
}
