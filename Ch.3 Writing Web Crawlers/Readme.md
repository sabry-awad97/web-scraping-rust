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
