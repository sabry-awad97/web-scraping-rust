# Web Scraping

Web scraping is the process of extracting data from websites. Rust is a programming language known for its focus on safety and performance. Web scraping with Rust can be accomplished using libraries like "reqwest" for making HTTP requests and "scraper" for parsing HTML content. Let's dive into the steps and concepts involved in web scraping with Rust.

## An Introduction to reqwest

### What is Reqwest?

Reqwest is a popular HTTP client library in Rust that allows you to make HTTP requests to web servers. It's commonly used for tasks such as sending GET and POST requests, handling responses, and interacting with APIs. Reqwest provides a user-friendly interface and supports various features like handling cookies, timeouts, and custom headers.

### Basic Usage

To get started with Reqwest, you'll need to include it as a dependency in your Rust project's `Cargo.toml` file:

```toml
[dependencies]
reqwest = { version = "0.11.20", features = ["json"] }
tokio = { version = "1.32.0", features = ["full"] }
```

Here's a simple example of how you might use Reqwest to send a GET request to a website and retrieve its content:

```rs
use reqwest::Client;

#[tokio::main]
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let response = client
        .get("http://pythonscraping.com/pages/page1.html")
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.text().await?;

        println!("Body: {}", body);
    } else {
        println!("Request was not successful: {:?}", response.status());
    }

    Ok(())
}
```

## An Introduction to Scraper Crate in Rust

### What is Scraper Crate?

Scraper Crate is a powerful tool used in the Rust programming language for web scraping. Web scraping is the process of extracting data from websites, usually for the purpose of analysis or storage. Rust is a programming language known for its focus on safety and performance. Scraper Crate provides developers with the tools and libraries needed to create web scrapers efficiently.

### Why Use Scraper Crate?

`Ease of Use`: Scraper Crate simplifies the process of sending HTTP requests and parsing HTML content, which are essential steps in web scraping.

`Concurrency`: Rust's built-in concurrency features, combined with Scraper Crate, enable you to create efficient and parallelized web scrapers, making the process faster.

`Safety`: Rust's ownership and borrowing system helps prevent common programming errors, ensuring your scraper is robust and reliable.

`Community Support`: Rust has an active community of developers, and Scraper Crate is well-maintained, ensuring you have access to ongoing support and updates.

### Basic Usage of Scraper

To use Scraper Crate, you need to include it as a dependency in your Rust project's `Cargo.toml` file:

```toml
scraper = "0.17.1"
```

Here's an example of using "scraper" to extract all the text from `<h1>` elements on a webpage:

```rs
use scraper::{Html, Selector};

fn main() {
    let html = r#"
        <html>
            <body>
                <h1>Hello</h1>
                <h1>World</h1>
            </body>
        </html>
    "#;

    let document = Html::parse_document(html);
    let selector = Selector::parse("h1").unwrap();

    for h1 in document.select(&selector) {
        println!("{}", h1.text().collect::<String>());
    }
}
```
