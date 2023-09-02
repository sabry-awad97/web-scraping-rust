use reqwest::Client;
use scraper::Html;
use scraper::Selector;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),
}

#[derive(Default)]
pub struct WebFetcher {
    client: Client,
}

impl WebFetcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn fetch_url(&self, url: &str) -> Result<String, AppError> {
        let response = self.client.get(url).send().await?;
        let body = response.text().await?;
        Ok(body)
    }
}

fn generate_ngrams(text: &str, n: usize) -> Vec<String> {
    let tokens: Vec<&str> = text.split_whitespace().collect();
    let mut ngrams = Vec::new();

    for i in 0..(tokens.len() - n + 1) {
        let ngram: Vec<&str> = tokens[i..(i + n)].to_vec();
        ngrams.push(ngram.join(" "));
    }

    ngrams
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let url = "https://en.wikipedia.org/wiki/Rust_(programming_language)";
    let web_fetcher = WebFetcher::new();
    let body = web_fetcher.fetch_url(url).await?;
    let fragment = Html::parse_document(&body);

    let content_selector = Selector::parse("div#mw-content-text").unwrap();
    let mut content = String::new();
    for element in fragment.select(&content_selector) {
        content.push_str(&element.text().collect::<String>());
    }

    // Generate 2-grams from the content
    let n = 2;
    let ngrams = generate_ngrams(&content, n);

    // Print the first 10 2-grams and their count
    println!("{:?}", &ngrams[0..10]);
    println!("2-grams count is: {}", ngrams.len());

    Ok(())
}
