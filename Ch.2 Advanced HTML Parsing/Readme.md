# Advanced HTML Parsing

HTML parsing is the process of extracting structured data from HTML documents.

## Extracting Elements by Class Name

When working with HTML documents, elements are often given specific classes to group them based on their purpose or styling. To extract elements by their class name, you can use CSS selectors. CSS selectors allow you to target elements with specific attributes or classes.

```rs
use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonscraping.com/pages/warandpeace.html";
    let html = fetch_html(url).await?;

    let class_name = "green";
    if let Some(names) = get_elements_with_class(&html, class_name) {
        for name in names {
            println!("{}", name);
        }
    } else {
        println!("No names found.");
    }

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn get_elements_with_class(html: &str, class_name: &str) -> Option<Vec<String>> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(&format!(".{}", class_name)).unwrap();

    let names: Vec<String> = document
        .select(&selector)
        .map(|element| element.text().collect())
        .collect();

    if names.is_empty() {
        None
    } else {
        Some(names)
    }
}
```

## Extracting Tags

```rs
use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonscraping.com/pages/warandpeace.html";
    let html = fetch_html(url).await?;

    let headings = find_all_tags(&html, "h1, h2, h3, h4, h5, h6");
    for heading in headings {
        println!("{}", heading);
    }

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn find_all_tags(html: &str, tags: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(tags).unwrap();

    let tags: Vec<String> = document
        .select(&selector)
        .map(|element| element.text().collect())
        .collect();

    tags
}
```

## Extracting Tags with Classes

```rs
use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonscraping.com/pages/warandpeace.html";
    let html = fetch_html(url).await?;

    let spans = find_all_tags_with_classes(&html, "span", &["green", "red"]);
    for span in spans {
        println!("{}", span);
    }

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn find_all_tags_with_classes(html: &str, tag: &str, class_names: &[&str]) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = create_selector_for_maybe_classes(tag, class_names);
    let spans: Vec<String> = document
        .select(&selector)
        .map(|element| element.text().collect())
        .collect();

    spans
}

fn create_selector_for_maybe_classes(element_name: &str, class_names: &[&str]) -> Selector {
    let mut css_selector = String::from(element_name);
    for (index, class) in class_names.iter().enumerate() {
        if index == 0 {
            css_selector.push_str(&format!(".{}", class));
        } else {
            css_selector.push_str(&format!(", .{}", class));
        }
    }

    Selector::parse(&css_selector).unwrap()
}

fn create_selector_for_classes(element_name: &str, class_names: &[&str]) -> Selector {
    let mut css_selector = String::from(element_name);
    for class in class_names {
        css_selector.push_str(&format!(".{}", class));
    }

    Selector::parse(&css_selector).unwrap()
}
```

## Counting Text Occurrences

```rs
use scraper::Html;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonscraping.com/pages/warandpeace.html";
    let html = fetch_html(url).await?;

    let target_text = "the prince";
    let count = count_text_occurrences(&html, target_text);

    println!("Number of occurrences: {}", count);

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn count_text_occurrences(html: &str, target_text: &str) -> usize {
    let document = Html::parse_document(html);
    let count = document
        .root_element()
        .text()
        .filter(|text| text.contains(target_text))
        .count();

    count
}
```
