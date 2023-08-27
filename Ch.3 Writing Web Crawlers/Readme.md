# Writing Web Crawlers

## Introduction to Web Crawlers

A web crawler, also known as a spider or web scraper, is a program that automatically navigates through websites and extracts information from them. This information can include text, images, links, and other data. Web crawlers are commonly used for various purposes, such as gathering data for search engines, monitoring website changes, and collecting data for research or analysis.

## How Web Crawlers Work

Web crawlers work by following these general steps:

1. `Start Seed URL`: The crawler begins with a starting URL, also known as a seed URL, which is the initial webpage it will visit.

1. `Download Webpage`: The crawler fetches the content of the webpage using HTTP requests. This content includes HTML, CSS, JavaScript, and other assets.

1. `Parse HTML`: The HTML content is parsed to extract important elements like links and data. This is done using libraries and tools like Beautiful Soup or Scrapy in Python.

1. `Follow Links`: The crawler identifies links in the parsed HTML and adds them to a queue of URLs to visit next.

1. `Repeat Process`: The crawler continues this process iteratively, visiting each URL in the queue, downloading webpages, parsing them, and extracting information.

1. `Data Storage`: Extracted data can be stored in a database, files, or other data storage systems for further processing.

## Traversing a Single Domain

```rs
use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://en.wikipedia.org/wiki/Kevin_Bacon";
    let html = fetch_html(url).await?;

    let href_extractor =
        |element: scraper::ElementRef| element.value().attr("href").map(String::from);

    let href_attributes = find_attributes(&html, "a", href_extractor);

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

fn find_attributes<T, F>(html: &str, element_tag: &str, attribute_extractor: F) -> Vec<T>
where
    F: Fn(scraper::ElementRef) -> Option<T>,
{
    let document = Html::parse_document(html);
    let selector = Selector::parse(element_tag).unwrap();

    let attributes: Vec<T> = document
        .select(&selector)
        .filter_map(attribute_extractor)
        .collect();

    attributes
}
```

### Extracting Matching Href Attributes

```rs
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
```
