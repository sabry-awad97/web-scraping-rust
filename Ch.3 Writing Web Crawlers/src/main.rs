use fancy_regex::Regex;
use rand::{thread_rng, Rng};
use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start_article = "/wiki/Kevin_Bacon";
    follow_random_links(start_article).await?;

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

async fn follow_random_links(start_article: &str) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let mut current_article = start_article.to_string();

    loop {
        let html = fetch_html(&format!("http://en.wikipedia.org{}", current_article)).await?;
        let links = find_links_in_body_content(&html);

        if links.is_empty() {
            break;
        }

        let random_index = rng.gen_range(0..links.len());
        let new_article = &links[random_index];
        println!("{}", new_article);

        current_article = new_article.to_string();
    }

    Ok(())
}

fn find_links_in_body_content(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let a_selector = Selector::parse("div#bodyContent a[href^='/wiki/']").unwrap();
    let href_regex = r"^/wiki/((?!:).)*$";
    let regex = Regex::new(href_regex).unwrap();
    let links: Vec<String> = document
        .select(&a_selector)
        .filter_map(|element| element.value().attr("href").map(String::from))
        .filter(|href| regex.is_match(href).unwrap_or(false))
        .collect();

    links
}
