use async_trait::async_trait;
use csv::{Error as CsvError, ReaderBuilder};
use reqwest::{Client, Error as ReqwestError};
use std::collections::HashMap;
use std::io::Cursor;
use std::string::FromUtf8Error;

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] ReqwestError),

    #[error("UTF-8 decoding error: {0}")]
    Utf8Decoding(#[from] FromUtf8Error),

    #[error("CSV error: {0}")]
    Csv(#[from] CsvError),

    #[error("JSON serialization error: {0}")]
    JsonSerialization(#[from] serde_json::Error),
}

#[async_trait]
trait HtmlFetcher {
    async fn fetch_html(&self, url: &str) -> Result<String, AppError>;
}

struct ReqwestHtmlFetcher {
    client: Client,
}

impl ReqwestHtmlFetcher {
    fn new() -> Self {
        let client = Client::new();
        Self { client }
    }
}

#[async_trait]
impl HtmlFetcher for ReqwestHtmlFetcher {
    async fn fetch_html(&self, url: &str) -> Result<String, AppError> {
        let response = self.client.get(url).send().await?;
        let text_content = response.text().await?;
        Ok(text_content)
    }
}

trait CsvParser {
    fn parse_csv(&self, csv_text: &str) -> Result<(), AppError>;
}

struct CsvReader;

impl CsvParser for CsvReader {
    fn parse_csv(&self, csv_text: &str) -> Result<(), AppError> {
        let mut csv_reader = ReaderBuilder::new().from_reader(Cursor::new(csv_text));

        // Print field names
        if let Ok(field_names) = csv_reader.headers() {
            println!("field names: {}", field_names.iter().collect::<Vec<_>>().join(" | "));
        }

        println!("---------------------");

        // Print rows as dictionaries
        for result in csv_reader.deserialize::<HashMap<String, serde_json::Value>>() {
            let record = result?;
            let json_object = serde_json::to_value(&record)?;
            println!("{}", json_object);
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let html_fetcher = ReqwestHtmlFetcher::new();
    let csv_parser = CsvReader;
    let csv_url = "http://pythonscraping.com/files/MontyPythonAlbums.csv";

    let csv_text = html_fetcher.fetch_html(csv_url).await?;
    csv_parser.parse_csv(&csv_text)?;

    Ok(())
}
