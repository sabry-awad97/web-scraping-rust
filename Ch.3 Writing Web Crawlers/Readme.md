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

### Exploring Wikipedia: Random Article Hopping

```rs
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
```

## Crawling an Entire Site

Web scrapers that traverse an entire site are good for many things, including the following:

1. **Generating a site map**
1. **Gathering data**

Crawling an entire website involves systematically visiting and extracting information from all the pages within the website. This process is often used by search engines to index web content. Let's explore how to approach crawling an entire site.

### Understanding Site Crawling

Crawling an entire website involves systematically visiting and extracting information from all the pages within the website. This process is often used by search engines to index web content. Let's explore how to approach crawling an entire site.

### Managing URLs and Depth

When crawling a site, it's important to manage the URLs you've visited and the depth of your crawl. Here's a basic approach:

1. **URL Queue**: Maintain a queue of URLs to visit. Start with the seed URL (the homepage), and enqueue its links for further exploration.

1. **Visited URLs Set**: Keep a set of visited URLs to avoid revisiting the same page.

1. **Depth Control**: Decide whether you want to limit the crawl depth. For instance, you might choose to crawl only up to a certain number of levels deep.

### Recursive Crawling Algorithm

Here's a high-level algorithm for crawling a site:

1. Enqueue the seed URL.
1. While the URL queue is not empty and the desired depth is not reached:
   - Dequeue a URL.
   - If the URL is not in the visited set:
     - Fetch the webpage content.
     - Parse the HTML to extract information.
     - Enqueue the links found on the page.
     - Add the URL to the visited set.

### Dealing with Challenges

Crawling entire sites can present challenges:

1. **Robots.txt**: Some sites have a robots.txt file that specifies which pages should not be crawled. Respect this file to avoid legal and ethical issues.

1. **Dynamic Content**: Some websites use JavaScript to load content dynamically. To crawl such sites, you might need to use headless browsers like Puppeteer.

1. **Duplicate Content**: Be cautious of duplicate content and ensure you're not revisiting the same page multiple times.

### Considerations for Large Sites

Crawling large sites requires efficiency:

1. **Parallelism**: Use multiple threads or asynchronous programming to speed up the crawling process.

1. **Rate Limiting**: Some sites might block or throttle excessive requests. Implement rate limiting to avoid being blocked.

1. **Data Storage**: Decide how you'll store the crawled data. A database is often used to organize and manage the extracted information.

## Recursive Wikipedia Crawling

```rs
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
```

## Collecting Data Across an Entire Site

```rs
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
```

## Handling Redirects

```rs
use std::error::Error;

use reqwest::{redirect, Client, Response};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://github.com";
    let response = fetch_url_with_redirect_handling(url).await?;

    println!("Final URL after following redirects: {}", response.url());
    println!("Response status: {}", response.status());
    println!("Response body:\n{}", response.text().await?);

    Ok(())
}

async fn fetch_url_with_redirect_handling(url: &str) -> Result<Response, reqwest::Error> {
    // Default will follow redirects up to a maximum of 10.
    let client = Client::builder()
        .redirect(redirect::Policy::limited(5))
        .build()?;

    let response = client.get(url).send().await?;
    Ok(response)
}
```

## Crawling Across the Internet

```rs
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
```

## Collect all External Links from a Site

```rs
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
```
