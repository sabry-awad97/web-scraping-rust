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
