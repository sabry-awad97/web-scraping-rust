use std::string::FromUtf8Error;

use async_trait::async_trait;

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("UTF-8 decoding error: {0}")]
    Utf8Decoding(#[from] FromUtf8Error),
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
        let bytes = response.bytes().await?;
        let text_content = String::from_utf8(bytes.to_vec())?;
        Ok(text_content)
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let web_scraper = ReqwestWebScraper;
    let text_url = "http://www.pythonscraping.com/pages/warandpeace/chapter1-ru.txt";

    let text_content = web_scraper.fetch_html(text_url).await?;

    println!("{}", text_content);
    Ok(())
}
