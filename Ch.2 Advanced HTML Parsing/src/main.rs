use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonscraping.com/pages/page3.html";
    let html = fetch_html(url).await?;

    let matching_elements =
        find_elements_with_closure(&html, |element| element.value().attrs.len() == 2);

    for element in matching_elements {
        println!("{}", element);
    }

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn find_elements_with_closure<F>(html: &str, closure: F) -> Vec<String>
where
    F: Fn(&scraper::ElementRef) -> bool,
{
    let document = Html::parse_document(html);
    let matching_elements: Vec<String> = document
        .select(&Selector::parse("*").unwrap())
        .filter(|element| closure(element))
        .map(|element| element.text().collect())
        .collect();

    matching_elements
}
