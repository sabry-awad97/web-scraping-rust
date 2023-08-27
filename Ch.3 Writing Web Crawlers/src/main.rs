use fancy_regex::Regex;
use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://en.wikipedia.org/wiki/Kevin_Bacon";
    let html = fetch_html(url).await?;

    let href_regex = r"^/wiki/((?!:).)*$";
    let href_attributes = find_matching_href_attributes(&html, "div#bodyContent", href_regex);

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

fn find_matching_href_attributes(html: &str, div_selector: &str, href_regex: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let div_selector = Selector::parse(div_selector).unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let regex = Regex::new(href_regex).unwrap();

    let matching_href_attributes: Vec<String> = document
        .select(&div_selector)
        .flat_map(|div| div.select(&a_selector))
        .filter_map(|element| {
            let href = element.value().attr("href")?;
            if regex.is_match(href).unwrap_or(false) {
                Some(href.to_string())
            } else {
                None
            }
        })
        .map(String::from)
        .collect();

    matching_href_attributes
}
