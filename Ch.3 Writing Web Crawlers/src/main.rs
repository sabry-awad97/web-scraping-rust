use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://en.wikipedia.org/wiki/Kevin_Bacon";
    let html = fetch_html(url).await?;

    let href_extractor =
        |element: scraper::ElementRef| element.value().attr("href").map(String::from);

    let href_attributes = find_attributes(&html, "a", href_extractor);

    for href in href_attributes {
        println!("{}", href);
    }

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn find_attributes<T, F>(html: &str, element_tag: &str, attribute_extractor: F) -> Vec<T>
where
    F: Fn(scraper::ElementRef) -> Option<T>,
{
    let document = Html::parse_document(html);
    let selector = Selector::parse(element_tag).unwrap();

    let attributes: Vec<T> = document
        .select(&selector)
        .filter_map(attribute_extractor)
        .collect();

    attributes
}
