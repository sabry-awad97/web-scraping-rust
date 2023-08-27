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
