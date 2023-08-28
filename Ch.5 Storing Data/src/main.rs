use csv::Writer;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://en.wikipedia.org/wiki/Comparison_of_text_editors";
    let html = fetch_html(url).await?;

    let table_selector = "table.wikitable";
    let rows = find_table_rows(&html, table_selector);

    let csv_file_path = "editors.csv";
    let csv_file = File::create(csv_file_path)?;

    let mut csv_writer = Writer::from_writer(csv_file);

    for row in rows {
        csv_writer.write_record(row)?;
    }

    csv_writer.flush()?;

    println!("CSV file written successfully.");

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn find_table_rows<'a>(html: &'a str, selector: &'a str) -> Vec<Vec<String>> {
    let document = Html::parse_document(html);
    let table_selector = Selector::parse(selector).unwrap();
    let row_selector = Selector::parse("tr").unwrap();

    let selected_elements: std::iter::Skip<scraper::html::Select<'_, '_>> =
        document.select(&table_selector).skip(1);

    let mut table_rows = Vec::new();

    for table in selected_elements {
        let row_data: Vec<String> = table
            .select(&row_selector)
            .flat_map(|row| {
                row.select(&Selector::parse("td").unwrap())
                    .map(|cell| cell.text().collect::<String>())
                    .collect::<Vec<_>>()
            })
            .collect();

        table_rows.push(row_data);
    }

    table_rows
}
