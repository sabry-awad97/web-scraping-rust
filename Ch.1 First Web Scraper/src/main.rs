use reqwest::Error as ReqwestError;
use scraper::{error::SelectorErrorKind, Html, Selector};
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("HTTP request error: {0}")]
    Http(#[from] ReqwestError),
    #[error("Selector parsing error: {0}")]
    SelectorParse(#[from] SelectorErrorKind<'static>),
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let title = get_title("http://www.pythonscraping.com/pages/page1.html").await?;

    if let Some(title) = title {
        println!("{}", title);
    } else {
        println!("Title could not be found");
    }

    Ok(())
}

async fn get_title(url: &str) -> Result<Option<String>, AppError> {
    if let Some(body) = get_site_html(url).await? {
        let document = Html::parse_document(&body);
        let h1_selector = Selector::parse("body h1")?;

        if let Some(h1_element) = document.select(&h1_selector).next() {
            let title = h1_element.text().collect::<String>();
            Ok(Some(title))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

async fn get_site_html(url: &str) -> Result<Option<String>, AppError> {
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        Ok(Some(body))
    } else {
        Ok(None)
    }
}
