use regex::Regex;
use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonscraping.com/pages/page3.html";
    let html = fetch_html(url).await?;

    let img_src_regex = r"\.\.\/img\/gifts\/img.*\.jpg";
    let src_attributes = find_images_with_src_regex(&html, img_src_regex);
    for src in src_attributes {
        println!("{}", src);
    }

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn find_images_with_src_regex(html: &str, img_src_regex: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let img_selector = Selector::parse("img[src]").unwrap();
    let regex = Regex::new(img_src_regex).unwrap();

    let src_attributes: Vec<String> = document
        .select(&img_selector)
        .filter_map(|element| element.value().attr("src"))
        .filter(|src| regex.is_match(src))
        .map(String::from)
        .collect();

    src_attributes
}
