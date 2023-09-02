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
