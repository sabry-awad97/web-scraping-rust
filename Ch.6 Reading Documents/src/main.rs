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
