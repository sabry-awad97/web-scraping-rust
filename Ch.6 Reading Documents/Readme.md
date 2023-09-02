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

## Reading CSV

```rs
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
```

### Print rows as dictionaries

```rs
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
```

### Reading PDF

```rs
use async_trait::async_trait;
use reqwest::Client;
use std::error::Error;

// Define a trait for PDF fetching
#[async_trait]
trait PdfFetcher {
    async fn fetch_pdf(&self, url: &str) -> Result<Vec<u8>, Box<dyn Error>>;
}

// Implement the PdfFetcher trait for a specific PDF fetcher
struct PdfUrlFetcher {
    client: Client,
}

impl PdfUrlFetcher {
    fn new() -> Self {
        let client = Client::new();
        Self { client }
    }
}

#[async_trait]
impl PdfFetcher for PdfUrlFetcher {
    async fn fetch_pdf(&self, url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        // Fetch the PDF content using reqwest
        let pdf_response = self.client.get(url).send().await?;
        let pdf_bytes = pdf_response.bytes().await?;
        Ok(pdf_bytes.to_vec())
    }
}

// Define a trait for PDF text extraction
trait PdfTextExtractorService {
    fn extract_text(&self, pdf_data: &[u8]) -> Result<String, Box<dyn Error>>;
}

// Implement the PdfTextExtractorService trait for a specific extractor
struct PdfExtractor;

impl PdfTextExtractorService for PdfExtractor {
    fn extract_text(&self, pdf_data: &[u8]) -> Result<String, Box<dyn Error>> {
        let pdf_text = pdf_extract::extract_text_from_mem(pdf_data)?;

        Ok(pdf_text)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pdf_url = "http://pythonscraping.com/pages/warandpeace/chapter1.pdf";
    let pdf_fetcher = PdfUrlFetcher::new();
    let pdf_data = pdf_fetcher.fetch_pdf(pdf_url).await?;

    let pdf_extractor = PdfExtractor;
    let pdf_text = pdf_extractor.extract_text(&pdf_data)?;

    println!("{}", pdf_text);
    Ok(())
}
```

## Reading Text Content from a Word Document

```rs
use async_trait::async_trait;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use quick_xml::Error as QuickXmlError;
use reqwest::Client;
use std::error::Error;
use std::io::{self, Read};
use zip::ZipArchive;

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Quick XML error: {0}")]
    QuickXml(#[from] QuickXmlError),

    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
}

#[async_trait]
trait DocxFetcher {
    async fn fetch_docx(&self, url: &str) -> Result<Vec<u8>, AppError>;
}

struct ReqwestDocxFetcher {
    client: Client,
}

impl ReqwestDocxFetcher {
    fn new() -> Self {
        let client = Client::new();
        Self { client }
    }
}

#[async_trait]
impl DocxFetcher for ReqwestDocxFetcher {
    async fn fetch_docx(&self, url: &str) -> Result<Vec<u8>, AppError> {
        let response = self.client.get(url).send().await?;
        let docx_bytes = response.bytes().await?.to_vec();
        Ok(docx_bytes)
    }
}

trait DocxReader {
    fn read_docx(&self, docx_content: &[u8]) -> Result<String, AppError>;
}

struct DocxTextReader;

impl DocxReader for DocxTextReader {
    fn read_docx(&self, docx_content: &[u8]) -> Result<String, AppError> {
        // Wrap the Word file in a Read cursor
        let cursor = io::Cursor::new(docx_content);

        // Open the Word document as a Zip archive
        let mut archive = ZipArchive::new(cursor)?;

        // Read the content of 'word/document.xml'
        let mut xml_content = String::new();

        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)?;
            if entry.name() == "word/document.xml" {
                entry.read_to_string(&mut xml_content)?;
                break;
            }
        }

        Ok(xml_content)
    }
}

trait DocxExtractor {
    fn extract_text(&self, docx_content: &str) -> Result<String, AppError>;
}

struct QuickXmlDocxTextExtractor;

impl DocxExtractor for QuickXmlDocxTextExtractor {
    fn extract_text(&self, xml_content: &str) -> Result<String, AppError> {
        let mut reader = Reader::from_str(xml_content);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut inside_w_t = false;

        let mut extracted_text = String::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    if e.name().as_ref() == b"w:t" {
                        inside_w_t = true;
                    }
                }
                Ok(Event::Text(e)) => {
                    if inside_w_t {
                        let text = &e.unescape()?;
                        extracted_text.push_str(text);
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name().as_ref() == b"w:t" {
                        inside_w_t = false;
                    }
                }
                Ok(Event::Eof) => break,
                _ => {}
            }

            buf.clear();
        }

        Ok(extracted_text)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let docx_fetcher = ReqwestDocxFetcher::new();
    let reader = DocxTextReader;

    let docx_url = "http://pythonscraping.com/pages/AWordDocument.docx";

    let docx_data = docx_fetcher.fetch_docx(docx_url).await?;
    let xml_content = reader.read_docx(&docx_data)?;

    let text_extractor = QuickXmlDocxTextExtractor;
    let extracted_text = text_extractor.extract_text(&xml_content)?;

    println!("{}", extracted_text);

    Ok(())
}
```
