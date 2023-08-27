use scraper::{ElementRef, Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.pythonscraping.com/pages/page3.html";
    let html = fetch_html(url).await?;

    let table_id = "giftList";
    let siblings = iterate_table_next_siblings(&html, table_id);
    for sibling in siblings {
        println!("{}", sibling);
    }

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn iterate_table_next_siblings(html: &str, table_id: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(&format!("#{} tr", table_id)).unwrap();

    if let Some(table_row) = document.select(&selector).next() {
        let siblings: Vec<String> = table_row
            .next_siblings()
            .filter_map(ElementRef::wrap)
            .map(|element| {
                let selector = Selector::parse("td").unwrap();
                let cells = element.select(&selector).skip(1);
                cells
                    .map(|cell| cell.text().collect::<String>())
                    .collect::<Vec<_>>()
                    .join("\t")
            })
            .collect();

        siblings
    } else {
        Vec::new()
    }
}
