use scraper::{Html, Selector};
use std::collections::HashSet;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start_page = "";
    follow_links_recursive(start_page).await?;

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

async fn follow_links_recursive(seed_url: &str) -> Result<(), Box<dyn Error>> {
    let mut pages = HashSet::new();
    let mut to_visit = vec![seed_url.to_string()];

    while let Some(current_page) = to_visit.pop() {
        let html = fetch_html(&format!("http://en.wikipedia.org{}", current_page)).await?;
        let links = find_links(&html);

        for link in links {
            if !pages.contains(&link) {
                println!("Visiting: {}", link);
                pages.insert(link.clone());
                to_visit.push(link);
            }
        }
    }

    Ok(())
}

fn find_links(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let a_selector = Selector::parse("a[href^='/wiki/']").unwrap();
    let href_extractor =
        |element: scraper::ElementRef| element.value().attr("href").map(String::from);

    let links: Vec<_> = document
        .select(&a_selector)
        .filter_map(href_extractor)
        .collect();

    links
}
