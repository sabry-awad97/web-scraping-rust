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

## Extracting with Id

```rs
use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonscraping.com/pages/warandpeace.html";
    let html = fetch_html(url).await?;

    let elements = find_elements_by_id(&html, "text");
    for element in elements {
        println!("{}", element);
    }

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn find_elements_by_id(html: &str, id_value: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(&format!("#{}", id_value)).unwrap();

    let elements: Vec<String> = document
        .select(&selector)
        .map(|element| element.text().collect())
        .collect();

    elements
}
```

## Navigating Trees in HTML Parsing

When you parse an HTML document, it's represented as a tree-like structure known as the Document Object Model (DOM). This tree structure consists of various elements and their relationships. Navigating this tree allows you to access and manipulate different parts of the HTML document.

### Traversing Parents and Children

When navigating a DOM tree, you can move between parent and child elements. For instance, in `scraper`, you can use methods like `.children()` and `.parent()` to traverse the tree:

- `.parent()`: This method returns the parent element of the current element. It's useful when you want to move up the tree and access the container element that holds the current element.

- `.children()`: This method returns an iterator over the child elements of the current element. It's useful when you want to access or manipulate the children of an element.

```rs
use scraper::{Html, Selector};

fn main() {
    let html = r#"
        <div class="container">
            <h1>Hello, <span class="name">Sabry</span>!</h1>
            <p>Welcome to our website.</p>
        </div>
    "#;

    let document = Html::parse_document(html);
    let container_selector = Selector::parse(".container").unwrap();

    // Get the container element
    let container_element = document.select(&container_selector).next().unwrap();

    // Using .children() to iterate over child elements
    for child_element in container_element.children() {
        println!("Child element: {:#?}", child_element);
    }

    // Using .parent() to get the parent element
    let h1_selector = Selector::parse("h1").unwrap();
    let h1_element = container_element.select(&h1_selector).next().unwrap();
    let parent_element = h1_element.parent().unwrap();
    println!("Parent element of h1: {:#?}", parent_element);
}
```

## Structured printing of the HTML content

```rs
use scraper::{ElementRef, Html};

fn main() {
    let html = r#"
        <body>
            <div class="wrapper">
                <h1>Title</h1>
                <div class="content">
                    <table id="giftList">
                        <tr>
                            <th>Header 1</th>
                            <th>Header 2</th>
                            <th>Header 3</th>
                            <th>Header 4</th>
                        </tr>
                        <tr class="gift" id="gift1">
                            <td>Data 1</td>
                            <td>Data 2</td>
                            <span class="excitingNote">Note</span>
                            <td>Data 3</td>
                            <td><img src="img_url" alt="Gift Image"></td>
                        </tr>
                    </table>
                </div>
                <div class="footer"></div>
            </div>
        </body>
    "#;

    let document = Html::parse_document(html);
    let mut depth = 0;
    print_node(&document.root_element(), &mut depth);
}

fn print_node(element: &scraper::ElementRef, depth: &mut usize) {
    let indent = "    ".repeat(*depth);
    let tag_name = element.value().name();
    let class_names = element
        .value()
        .classes()
        .map(|c| format!(".{}", c))
        .collect::<Vec<_>>();

    let id = element
        .value()
        .id()
        .map(|id| format!("#{}", id))
        .unwrap_or_default();

    println!("{}— {}{}{}", indent, tag_name, class_names.join(""), id);

    *depth += 1;

    for child_element in element.children().filter_map(ElementRef::wrap) {
        print_node(&child_element, depth);
    }

    *depth -= 1;
}
```

## Extracting Table Data

```rs
pub mod html_tree_printer;

use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonscraping.com/pages/page3.html";
    let html = fetch_html(url).await?;

    let table_id = "giftList";
    let children = iterate_table_children(&html, table_id);
    for child in children {
        println!("{}", child);
    }

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn iterate_table_children(html: &str, table_id: &str) -> Vec<String> {
    let document = Html::parse_document(html);

    let selector = Selector::parse(&format!("#{} tr", table_id)).unwrap();

    let children: Vec<String> = document
        .select(&selector)
        .map(|element| element.text().collect())
        .collect();

    children
}
```
