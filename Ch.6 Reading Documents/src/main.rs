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
