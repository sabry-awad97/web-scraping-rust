# Cleaning Data

## N-gram

`N-grams` are contiguous sequences of 'n' items (words, characters, or symbols) from a given text or speech. In the context of NLP, these items are typically words, but they can also be characters or other linguistic units. They are widely used in natural language processing, text analysis, and machine learning tasks. N-grams are used to capture the context and relationships between words or characters in a given sequence of text.

Here's a breakdown of different types of N-grams:

1. `Unigrams (1-grams)`: These are single words. For example, in the sentence "I love NLP," the unigrams would be ["I", "love", "NLP"].

1. `Bigrams (2-grams)`: These consist of pairs of consecutive words. In the same sentence, the bigrams would be ["I love", "love NLP"].

1. `Trigrams (3-grams)`: These consist of sequences of three consecutive words. In the sentence, the trigrams would be ["I love NLP"].

1. `N-grams (N > 3)`: These are sequences of 'n' consecutive words, where 'n' can be any positive integer greater than 3.

To implement `N-gram` generation:

1. Tokenize the input text into words or characters.
1. Create sliding windows of the desired N-gram size.
1. Collect and store the N-grams.

Here's a basic example of generating N-grams of words (unigrams, bigrams, and trigrams) from a text:

```rs
fn main() {
    let input_text = "This is a sample text for N-gram generation.";

    // Generate unigrams (1-grams)
    let unigrams = generate_ngrams(input_text, 1);
    println!("Unigrams: {:?}", unigrams);

    // Generate bigrams (2-grams)
    let bigrams = generate_ngrams(input_text, 2);
    println!("Bigrams: {:?}", bigrams);

    // Generate trigrams (3-grams)
    let trigrams = generate_ngrams(input_text, 3);
    println!("Trigrams: {:?}", trigrams);
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
```

```rs
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

fn generate_ngrams(text: &str, n: usize) -> Vec<Vec<String>> {
    let mut ngrams: Vec<Vec<String>> = Vec::new();

    for sentence in clean_input(text) {
        ngrams.extend(generate_ngrams_from_sentence(&sentence, n))
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
```

## Data Normalization

```rs
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
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

pub struct TextProcessor;

impl TextProcessor {
    pub fn clean_sentence(sentence: &str) -> Vec<String> {
        let cleaned_words: Vec<String> = sentence
            .split_whitespace()
            .map(|word| {
                let word_trimmed =
                    word.trim_matches(|c: char| c.is_whitespace() || c.is_ascii_punctuation());
                String::from(word_trimmed)
            })
            .filter(|word| {
                word.len() > 1 || word.to_lowercase() == "a" || word.to_lowercase() == "i"
            })
            .collect();

        cleaned_words
    }

    pub fn clean_input(content: &str) -> Vec<Vec<String>> {
        let content_upper = content.to_uppercase();
        let content_no_newline = content_upper.replace('\n', " ");
        let content_bytes = content_no_newline.as_bytes();
        let content_ascii = String::from_utf8_lossy(content_bytes);
        let sentences: Vec<&str> = content_ascii.split(". ").collect();

        let cleaned_sentences: Vec<Vec<String>> = sentences
            .iter()
            .map(|sentence| Self::clean_sentence(sentence))
            .collect();

        cleaned_sentences
    }

    pub fn get_ngrams_from_sentence(sentence: &[String], n: usize) -> Vec<String> {
        sentence.windows(n).map(|window| window.join(" ")).collect()
    }

    pub fn get_ngrams(content: &str, n: usize) -> HashMap<String, usize> {
        let cleaned_content = Self::clean_input(content);
        let mut ngrams: HashMap<String, usize> = HashMap::new();

        for sentence in &cleaned_content {
            let new_ngrams = Self::get_ngrams_from_sentence(sentence, n);
            for ngram in new_ngrams {
                *ngrams.entry(ngram).or_insert(0) += 1;
            }
        }

        ngrams
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let url = "https://en.wikipedia.org/wiki/Rust_(programming_language)";
    let web_fetcher = WebFetcher::new();
    let body = web_fetcher.fetch_url(url).await?;
    let fragment = Html::parse_document(&body);

    let content_selector = Selector::parse("div#mw-content-text").unwrap();
    let content: String = fragment
        .select(&content_selector)
        .flat_map(|element| element.text())
        .collect();

    // Generate 2-grams from the content
    let n = 2;
    let ngrams = TextProcessor::get_ngrams(&content, n);

    // Print the n-grams and their counts
    for (ngram, count) in &ngrams {
        println!("N-gram: '{}', Count: {}", ngram, count);
    }

    Ok(())
}
```
