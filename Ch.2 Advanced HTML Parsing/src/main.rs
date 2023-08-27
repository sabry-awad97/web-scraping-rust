use scraper::{ElementRef, Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonscraping.com/pages/page3.html";
    let html = fetch_html(url).await?;

    let img_src = "../img/gifts/img1.jpg";
    if let Some(text) = find_previous_sibling_text(&html, img_src) {
        println!("{}", text);
    } else {
        println!("Previous sibling text not found.");
    }

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn find_previous_sibling_text(html: &str, img_src: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let img_selector = Selector::parse(&format!("img[src='{}']", img_src)).unwrap();

    if let Some(img_element) = document.select(&img_selector).next() {
        if let Some(prev_sibling) = img_element
            .parent()
            .and_then(|parent| parent.prev_sibling())
            .and_then(ElementRef::wrap)
        {
            return Some(prev_sibling.text().collect::<String>());
        }
    }

    None
}
