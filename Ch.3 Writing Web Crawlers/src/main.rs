use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use scraper::{Html, Selector};
use std::error::Error;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let starting_site = "http://oreilly.com";
    follow_external_only(starting_site).await?;

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

async fn follow_external_only(starting_site: &str) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let mut current_site = starting_site.to_string();
    let url = Url::parse(&current_site)?;

    loop {
        let html = fetch_html(&current_site).await?;
        let external_links = find_external_links(&html, &url.origin());

        if external_links.is_empty() {
            println!("No external links, looking around the site for one");
            let internal_links = find_internal_links(&html, url.origin());
            if let Some(internal_link) = internal_links.choose(&mut rng) {
                current_site = internal_link.to_string();
            } else {
                break;
            }
        } else {
            let random_index = rng.gen_range(0..external_links.len());
            let external_link = &external_links[random_index];
            println!("Random external link is: {}", external_link);
            current_site = external_link.to_string();
        }
    }

    Ok(())
}

fn find_internal_links(html: &str, domain: url::Origin) -> Vec<String> {
    let document = Html::parse_document(html);
    let a_selector = Selector::parse("a[href]").unwrap();
    let mut internal_links = Vec::new();

    for element in document.select(&a_selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(url) = Url::parse(href) {
                if url.origin() == domain {
                    internal_links.push(url.to_string());
                }
            }
        }
    }

    internal_links
}

fn find_external_links(html: &str, exclude_domain: &url::Origin) -> Vec<String> {
    let document = Html::parse_document(html);
    let a_selector = Selector::parse("a[href]").unwrap();
    let mut external_links = Vec::new();

    for element in document.select(&a_selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(url) = Url::parse(href) {
                if url.origin() != *exclude_domain {
                    external_links.push(url.to_string());
                }
            }
        }
    }

    external_links
}
