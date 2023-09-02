use std::collections::HashMap;

use fancy_regex::Regex;
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

fn clean_sentence(sentence: &str) -> Vec<String> {
    let words = sentence
        .split_whitespace()
        .map(|word| word.trim_matches(|c| char::is_whitespace(c) || char::is_ascii_punctuation(&c)))
        .filter(|word| word.len() > 1 || word.to_lowercase() == "a" || word.to_lowercase() == "i")
        .map(String::from)
        .collect();

    words
}

fn clean_input(content: &str) -> Vec<Vec<String>> {
    let re = Regex::new(r"\n|\[\d+\]").unwrap();
    let cleaned_content = re.replace_all(content, " ").to_string();
    let sentences: Vec<&str> = cleaned_content
        .split(". ")
        .map(|sentence| sentence.trim())
        .collect();

    let cleaned_sentences: Vec<Vec<String>> = sentences
        .iter()
        .map(|sentence| clean_sentence(sentence))
        .collect();

    cleaned_sentences
}

fn generate_ngrams_from_sentence(sentence: &[String], n: usize) -> Vec<Vec<String>> {
    let mut ngrams: Vec<Vec<String>> = Vec::new();

    if sentence.len() >= n {
        for i in 0..=(sentence.len() - n) {
            let ngram = sentence[i..(i + n)].to_vec();
            ngrams.push(ngram);
        }
    }

    ngrams
}

fn get_ngrams(content: &str, n: usize) -> HashMap<String, usize> {
    let cleaned_content = clean_input(content);
    let mut ngrams: HashMap<String, usize> = HashMap::new();

    for sentence in cleaned_content {
        let sentence_ngrams = generate_ngrams_from_sentence(&sentence, n);
        for ngram in sentence_ngrams {
            let ngram_str = ngram.join(" ");
            *ngrams.entry(ngram_str).or_insert(0) += 1;
        }
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
    let ngrams = get_ngrams(&content, n);

    // Print the n-grams and their counts
    for (ngram, count) in &ngrams {
        println!("N-gram: '{}', Count: {}", ngram, count);
    }

    Ok(())
}
