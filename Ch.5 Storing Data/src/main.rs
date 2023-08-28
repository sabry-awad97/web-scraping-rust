use scraper::{Html, Selector};
use std::fs::File;
use std::io::Write;

#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("File operation failed: {0}")]
    FileError(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let url = "http://www.pythonscraping.com";
    let image_title = "logo01";
    let image_file_name = "logo.jpg";

    let html = fetch_html(url).await?;
    let image_location = find_image_location(&html, image_title);
    if let Some(image_url) = image_location {
        download_image(image_url, image_file_name).await?;
        println!("Image downloaded successfully.");
    } else {
        println!("Image not found on the page.");
    }

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, AppError> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn find_image_location(html: &str, title: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(&format!("img[title='{}']", title)).unwrap();

    if let Some(image_element) = document.select(&selector).next() {
        image_element.value().attr("src").map(String::from)
    } else {
        None
    }
}

async fn download_image(image_url: String, file_name: &str) -> Result<(), AppError> {
    let response = reqwest::get(&image_url).await?;
    let mut image_file = File::create(file_name)?;
    let image_bytes = response.bytes().await?;
    image_file.write_all(&image_bytes)?;
    Ok(())
}
