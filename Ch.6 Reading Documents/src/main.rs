use async_trait::async_trait;
use docx_rs::{read_docx, DocumentChild, ParagraphChild, ReaderError, RunChild};
use reqwest::Client;
use std::error::Error;
use std::io;

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Docx reader error: {0}")]
    Reader(#[from] ReaderError),
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

trait DocxTextExtractor {
    fn extract_text(&self, docx_content: &[u8]) -> Result<String, AppError>;
}

struct DocxExtractor;

impl DocxTextExtractor for DocxExtractor {
    fn extract_text(&self, docx_content: &[u8]) -> Result<String, AppError> {
        let docx = read_docx(docx_content)?;

        let mut text = String::new();
        for child in docx.document.children {
            if let DocumentChild::Paragraph(p) = child {
                for pc in p.children {
                    if let ParagraphChild::Run(r) = pc {
                        for rc in r.children {
                            if let RunChild::Text(t) = rc {
                                text.push_str(&t.text);
                            }
                        }
                    }
                }
            }
        }

        Ok(text)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let docx_fetcher = ReqwestDocxFetcher::new();
    let text_extractor = DocxExtractor;

    let docx_url = "http://pythonscraping.com/pages/AWordDocument.docx";

    let docx_data = docx_fetcher.fetch_docx(docx_url).await?;
    let text_content = text_extractor.extract_text(&docx_data)?;

    println!("{}", text_content);

    Ok(())
}
