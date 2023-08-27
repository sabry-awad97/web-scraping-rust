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

async fn follow_links_recursive(start_page: &str) -> Result<(), Box<dyn Error>> {
    let mut pages = HashSet::new();
    let mut to_visit = vec![start_page.to_string()];

    while let Some(current_page) = to_visit.pop() {
        let html = fetch_html(&format!("http://en.wikipedia.org{}", current_page)).await?;
        let (title, first_paragraph, edit_link) = extract_page_details(&html);

        println!("Title: {}", title);
        println!("First Paragraph: {}", first_paragraph);
        println!("Edit Link: {}", edit_link);
        println!("{}", "-".repeat(20));

        let links = find_links(&html);

        for link in links {
            if !pages.contains(&link) {
                pages.insert(link.clone());
                to_visit.push(link);
            }
        }
    }

    Ok(())
}

fn extract_page_details(html: &str) -> (String, String, String) {
    let document = Html::parse_document(html);
    let title_selector = Selector::parse("h1").unwrap();
    let first_paragraph_selector = Selector::parse("div#mw-content-text p").unwrap();
    let edit_link_selector = Selector::parse("#ca-edit span a").unwrap();

    let title = document
        .select(&title_selector)
        .next()
        .map(|element| element.text().collect())
        .unwrap_or(String::from("No Title"));

    let first_paragraph = document
        .select(&first_paragraph_selector)
        .next()
        .map(|element| element.text().collect())
        .unwrap_or(String::from("No First Paragraph"));

    let edit_link = document
        .select(&edit_link_selector)
        .next()
        .and_then(|element| element.value().attr("href").map(String::from))
        .unwrap_or(String::from("No Edit Link"));

    (title, first_paragraph, edit_link)
}

fn find_links(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let a_selector = Selector::parse("a[href^='/wiki/']").unwrap();
    let links: Vec<String> = document
        .select(&a_selector)
        .filter_map(|element| element.value().attr("href").map(String::from))
        .collect();

    links
}
