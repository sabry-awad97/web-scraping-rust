use async_trait::async_trait;
use csv::{Error as CsvError, ReaderBuilder};
use reqwest::{Client, Error as ReqwestError};
use std::string::FromUtf8Error;

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] ReqwestError),

    #[error("UTF-8 decoding error: {0}")]
    Utf8Decoding(#[from] FromUtf8Error),

    #[error("CSV error: {0}")]
    Csv(#[from] CsvError),
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
        let mut csv_reader = ReaderBuilder::new().from_reader(csv_text.as_bytes());

        for result in csv_reader.records() {
            let record = result?;
            for field in record.iter() {
                print!("{} | ", field);
            }
            println!();
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
