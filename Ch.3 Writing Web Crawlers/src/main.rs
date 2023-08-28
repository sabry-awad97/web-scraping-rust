use async_recursion::async_recursion;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::error::Error;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let starting_site = "http://oreilly.com";
    let mut all_ext_links = HashSet::new();
    let mut all_int_links = HashSet::new();

    get_all_external_links(starting_site, &mut all_ext_links, &mut all_int_links).await?;

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

#[async_recursion]
async fn get_all_external_links(
    site_url: &str,
    all_ext_links: &mut HashSet<String>,
    all_int_links: &mut HashSet<String>,
) -> Result<(), Box<dyn Error>> {
    let html = fetch_html(site_url).await?;
    let domain = Url::parse(site_url)?.origin();
    let internal_links = find_internal_links(&html, &domain);
    let external_links = find_external_links(&html, &domain);

    for link in external_links.iter() {
        if !all_ext_links.contains(link) {
            all_ext_links.insert(link.clone());
            println!("{}", link);
        }
    }

    for link in internal_links.iter() {
        if !all_int_links.contains(link) {
            all_int_links.insert(link.clone());
            get_all_external_links(link, all_ext_links, all_int_links).await?;
        }
    }

    Ok(())
}

fn find_internal_links(html: &str, domain: &url::Origin) -> Vec<String> {
    let document = Html::parse_document(html);
    let a_selector = Selector::parse("a[href]").unwrap();
    let mut internal_links = Vec::new();

    for element in document.select(&a_selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(url) = Url::parse(href) {
                if url.origin() == *domain {
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
