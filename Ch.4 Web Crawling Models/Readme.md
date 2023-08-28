# Web Crawling Models

## Different Approaches to Web Crawling

Web crawling can be approached using various models, each with its own advantages and considerations. Let's explore three common models: Breadth-First Crawling, Depth-First Crawling, and Iterative Deepening Crawling.

### 1\. Breadth-First Crawling

In the Breadth-First Crawling model, the crawler starts with the seed URL and systematically explores all linked pages at the same level before moving deeper. This approach ensures that pages closer to the seed URL are visited first, and it can provide a comprehensive overview of a website's content.

Advantages:

- Thorough coverage of the site.
- Ensures important pages are discovered early.
- Useful for search engines indexing entire sites.

Considerations:

- May lead to longer time before deep content is crawled.
- May generate a large number of requests quickly.

### 2\. Depth-First Crawling

Depth-First Crawling involves starting from the seed URL and following a path as deep as possible before backtracking. This approach can quickly explore specific sections of a site deeply.

Advantages:

- Efficient for finding content deeply nested within a site.
- Can be effective for discovering detailed information.

Considerations:

- Might miss important pages higher in the hierarchy.
- May not provide a holistic view of the site.

### 3\. Iterative Deepening Crawling

Iterative Deepening Crawling is a combination of breadth-first and depth-first approaches. It starts with breadth-first crawling for a certain depth, then switches to depth-first crawling for a deeper level, and so on. This model balances between thorough coverage and deep exploration.

Advantages:

- Balances between comprehensive coverage and deep exploration.
- Can be adaptable to different site structures.

Considerations:

- Requires fine-tuning the depth-switching strategy.
- Can be more complex to implement.

### Choosing the Right Model

The choice of crawling model depends on the goals of your crawl:

- Use Breadth-First Crawling for comprehensive coverage and indexing.
- Use Depth-First Crawling for in-depth analysis of specific sections.
- Use Iterative Deepening Crawling for a balanced approach.

## Avoiding Common Traps in Web Scraping

### The Pitfall of Relying Solely on Visible Data

When it comes to web scraping, a common mistake is to determine the data you want to collect solely based on what is visually present on a webpage. While this approach might seem logical, it can lead to incomplete, inaccurate, or unreliable data extraction.

### The Challenge of Dynamic Web Pages

Many modern websites use dynamic content that is loaded through JavaScript. This means that some of the data you're interested in might not be directly visible in the page source when it first loads. Instead, it might be loaded asynchronously or dynamically after the initial page load.

### Hidden Data and Structure

Websites often structure their data using HTML tags, classes, and attributes that are not visible to the end user. These hidden elements might contain valuable information, such as metadata, identifiers, or structured data, that's essential for accurate scraping.

### Solutions to the Trap

1. **Inspect Page Source:** While visible data is a starting point, it's crucial to inspect the entire page source using developer tools in your browser. This will reveal the structure of the page, even if some elements are not initially visible.
2. **Analyze Network Requests:** Monitor network requests in the browser's developer tools to identify additional data loaded dynamically. This can help you understand where the data comes from and how to access it programmatically.
3. **Use APIs:** Some websites offer APIs that provide structured and well-documented access to their data. APIs are often more reliable and efficient than scraping raw HTML.
4. **Dynamic Content Handling:** If the website heavily relies on JavaScript, consider using headless browsers like Puppeteer or tools like Selenium to interact with the page as a user would, triggering dynamic content to load.
5. **Inspect Hidden Elements:** Pay attention to HTML attributes, classes, and tags that might indicate the presence of hidden or structured data. This can provide clues about how to access the data you need.

### Example

Imagine a webpage listing products where the product names are visible, but the associated prices are loaded dynamically. Relying solely on visible data would result in missing price information. By inspecting the page source and monitoring network requests, you can identify how prices are loaded and adapt your scraping strategy accordingly.

## Crawling Sites through Links

```rs
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
```
