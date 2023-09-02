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
