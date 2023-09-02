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
