# Reading Documents

When scraping web pages, it's essential to understand document encoding to handle text correctly.

## Document Encoding

- Document encoding defines how characters are represented in a web page.
- Common encodings include UTF-8, ISO-8859-1, and others.
- It's crucial to identify and handle encoding to extract and display text correctly.

## Text

Absolutely, working with plain text files online is quite common, especially for sites that prioritize simplicity and accessibility.

```rs
use async_trait::async_trait;

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),
}

// WebScraper trait for fetching HTML
#[async_trait]
trait WebScraper {
    async fn fetch_html(&self, url: &str) -> Result<String, AppError>;
}

struct ReqwestWebScraper;

#[async_trait]
impl WebScraper for ReqwestWebScraper {
    async fn fetch_html(&self, url: &str) -> Result<String, AppError> {
        let response = reqwest::get(url).await?;
        let html = response.text().await?;
        Ok(html)
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let web_scraper = ReqwestWebScraper;
    let text_url = "http://www.pythonscraping.com/pages/warandpeace/chapter1.txt";

    let text_content = web_scraper.fetch_html(text_url).await?;

    println!("{}", text_content);
    Ok(())
}
```

### Filtering and Cleaning Text

Suppose you've extracted text with HTML tags, and you want to clean it:

```rs
fn main() {
    let raw_text = "<p>This is <strong>important</strong> text.</p>";

    // Remove HTML tags using a regex pattern.
    let cleaned_text = fancy_regex::Regex::new(r"<[^>]*>")
        .unwrap()
        .replace_all(raw_text, "");

    println!("{}", cleaned_text);
}
```
