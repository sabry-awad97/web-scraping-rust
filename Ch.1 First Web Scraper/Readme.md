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
