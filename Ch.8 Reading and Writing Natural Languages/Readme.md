# Reading and Writing Natural Languages

## Summarizing Data

```rs
use reqwest::Client;
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
    let url = "http://pythonscraping.com/files/inaugurationSpeech.txt";
    let web_fetcher = WebFetcher::new();
    let content = web_fetcher.fetch_url(url).await?;

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

## Markov Models

```rs
use rand::Rng;
use reqwest::Error;
use std::collections::HashMap;

struct TextFetcher;

impl TextFetcher {
    async fn fetch_text(url: &str) -> Result<String, Error> {
        let response = reqwest::get(url).await?;
        let body = response.text().await?;
        Ok(body)
    }
}

struct MarkovChainGenerator;

impl MarkovChainGenerator {
    fn word_list_sum(word_list: &HashMap<String, u32>) -> u32 {
        word_list.values().sum()
    }

    fn retrieve_random_word(word_list: &HashMap<String, u32>) -> &str {
        let mut rng = rand::thread_rng();
        let rand_index = rng.gen_range(1..=MarkovChainGenerator::word_list_sum(word_list));

        let mut current_index = 0;
        for (word, &value) in word_list.iter() {
            current_index += value;
            if current_index >= rand_index {
                return word;
            }
        }

        ""
    }

    fn build_word_dict(text: &str) -> HashMap<String, HashMap<String, u32>> {
        let mut word_dict = HashMap::new();
        let words: Vec<&str> = text.split_whitespace().collect();

        for i in 1..words.len() {
            let prev_word = words[i - 1].to_string();
            let current_word = words[i].to_string();

            let entry = word_dict.entry(prev_word.clone()).or_insert(HashMap::new());
            *entry.entry(current_word.clone()).or_insert(0) += 1;
        }

        word_dict
    }

    fn generate_markov_chain(
        word_dict: &HashMap<String, HashMap<String, u32>>,
        length: usize,
    ) -> Vec<String> {
        let mut chain: Vec<String> = vec!["I".to_string()];

        let default_hash = HashMap::new();
        for _ in 0..length {
            let prev_word = chain.last().unwrap().clone();
            let next_word = MarkovChainGenerator::retrieve_random_word(
                word_dict.get(&prev_word).unwrap_or(&default_hash),
            );
            chain.push(next_word.to_string());
        }

        chain
    }
}

struct MarkovChainPrinter;

impl MarkovChainPrinter {
    fn print_markov_chain(chain: &[String]) {
        println!("{}", chain.join(" "));
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let text_url = "http://pythonscraping.com/files/inaugurationSpeech.txt";
    let text = TextFetcher::fetch_text(text_url).await?;
    let word_dict = MarkovChainGenerator::build_word_dict(&text);

    let length = 100;
    let markov_chain = MarkovChainGenerator::generate_markov_chain(&word_dict, length);

    MarkovChainPrinter::print_markov_chain(&markov_chain);

    Ok(())
}
```
