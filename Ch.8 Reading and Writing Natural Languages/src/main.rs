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
    pub fn is_common(ngram: &[&str]) -> bool {
        let common_words = [
            "THE", "BE", "AND", "OF", "A", "IN", "TO", "HAVE", "IT", "I", "THAT", "FOR", "YOU",
            "HE", "WITH", "ON", "DO", "SAY", "THIS", "THEY", "IS", "AN", "AT", "BUT", "WE", "HIS",
            "FROM", "THAT", "NOT", "BY", "SHE", "OR", "AS", "WHAT", "GO", "THEIR", "CAN", "WHO",
            "GET", "IF", "WOULD", "HER", "ALL", "MY", "MAKE", "ABOUT", "KNOW", "WILL", "AS", "UP",
            "ONE", "TIME", "HAS", "BEEN", "THERE", "YEAR", "SO", "THINK", "WHEN", "WHICH", "THEM",
            "SOME", "ME", "PEOPLE", "TAKE", "OUT", "INTO", "JUST", "SEE", "HIM", "YOUR", "COME",
            "COULD", "NOW", "THAN", "LIKE", "OTHER", "HOW", "THEN", "ITS", "OUR", "TWO", "MORE",
            "THESE", "WANT", "WAY", "LOOK", "FIRST", "ALSO", "NEW", "BECAUSE", "DAY", "MORE",
            "USE", "NO", "MAN", "FIND", "HERE", "THING", "GIVE", "MANY", "WELL",
        ];

        for word in ngram {
            if common_words.contains(&word.to_uppercase().as_str()) {
                return true;
            }
        }

        false
    }

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
