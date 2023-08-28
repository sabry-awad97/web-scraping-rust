use fancy_regex::Regex;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug)]
struct Website<'a> {
    name: &'a str,
    url: &'a str,
    target_pattern: &'a str,
    absolute_url: bool,
    title_tag: &'a str,
    body_tag: &'a str,
}

#[derive(Debug)]
struct Content {
    url: String,
    title: String,
    body: String,
}

impl Content {
    fn print(&self) {
        println!("URL: {}", self.url);
        println!("TITLE: {}", self.title);
        println!("BODY:\n{}", self.body);
    }
}

struct Crawler<'a> {
    site: Website<'a>,
    visited: HashSet<String>,
}

impl<'a> Crawler<'a> {
    fn new(site: Website<'a>) -> Self {
        Self {
            site,
            visited: HashSet::new(),
        }
    }

    async fn get_page(&self, url: &str) -> Result<scraper::Html, reqwest::Error> {
        let response = reqwest::get(url).await?;
        let body = response.text().await?;
        Ok(Html::parse_document(&body))
    }

    fn safe_get(&self, page_obj: &scraper::Html, selector: &str) -> String {
        let select = Selector::parse(selector).unwrap();
        let selected_elems = page_obj.select(&select);
        selected_elems
            .map(|elem| elem.text().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    async fn parse(&mut self, url: &str) -> Result<(), Box<dyn Error>> {
        let bs = self.get_page(url).await?;
        let title = self.safe_get(&bs, self.site.title_tag);
        let body = self.safe_get(&bs, self.site.body_tag);

        if !title.is_empty() && !body.is_empty() {
            let content = Content {
                url: url.to_string(),
                title,
                body,
            };
            content.print();
        }

        Ok(())
    }

    async fn crawl(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Crawl website: {}", self.site.name);

        let html = self.get_page(self.site.url).await?;
        let target_selector = Selector::parse("a[href]")?;
        let regex = Regex::new(self.site.target_pattern)?;
        let target_pages = html
            .select(&target_selector)
            .filter_map(|element| element.value().attr("href"))
            .filter(|href| regex.is_match(href).unwrap_or(false))
            .collect::<Vec<_>>();

        for target_page in target_pages {
            if !self.visited.contains(target_page) {
                self.visited.insert(target_page.to_string());
                let target_url = if !self.site.absolute_url {
                    format!("{}{}", self.site.url, target_page)
                } else {
                    target_page.to_string()
                };
                self.parse(&target_url).await?;
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let reuters = Website {
        name: "Reuters",
        url: "https://www.reuters.com",
        target_pattern: "^(/world/)",
        absolute_url: false,
        title_tag: "h1",
        body_tag: "div.article-body__content__17Yit",
    };

    let mut crawler = Crawler::new(reuters);

    crawler.crawl().await?;
    Ok(())
}
