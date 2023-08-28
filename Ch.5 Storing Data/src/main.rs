use scraper::{Html, Selector};
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("File operation failed: {0}")]
    FileError(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let base_url = "http://pythonscraping.com";
    let download_directory = "downloaded";

    let html = fetch_html(base_url).await?;
    let download_list = find_download_list(&html);

    for download in download_list {
        if let Some(file_url) = get_absolute_url(base_url, &download) {
            println!("{}", file_url);
            let download_path = get_download_path(&file_url, download_directory)?;
            download_file(&file_url, &download_path).await?;
            println!("Downloaded: {}", download_path);
        }
    }

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, AppError> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn find_download_list(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("[src]").unwrap();

    document
        .select(&selector)
        .filter_map(|element| element.value().attr("src"))
        .map(String::from)
        .collect()
}

fn get_absolute_url(base_url: &str, source: &str) -> Option<String> {
    if source.starts_with("http://") || source.starts_with("https://") {
        Some(source.to_string())
    } else {
        Some(format!("{}/{}", base_url, source))
    }
}

fn sanitize_filename(filename: &str) -> String {
    filename.replace(['/', ':', '?'], "_")
}

fn get_download_path(file_url: &str, download_directory: &str) -> Result<String, AppError> {
    let filename = file_url
        .rsplit('/')
        .next()
        .unwrap_or("unknown_file")
        .split('?')
        .next()
        .expect("Invalid file URL");

    let sanitized_filename = sanitize_filename(filename);
    let full_path = format!("{}/{}", download_directory, sanitized_filename);
    let directory = Path::new(&full_path).parent().ok_or_else(|| {
        AppError::FileError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Parent directory not found",
        ))
    })?;

    if !directory.exists() {
        fs::create_dir_all(directory)?;
    }

    Ok(full_path.to_string())
}

async fn download_file(file_url: &str, file_path: &str) -> Result<(), AppError> {
    let response = reqwest::get(file_url).await?;
    let mut file = fs::File::create(file_path)?;
    let bytes = response.bytes().await?;
    file.write_all(&bytes)?;
    Ok(())
}
