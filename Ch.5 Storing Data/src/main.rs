use csv::WriterBuilder;
use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Send an HTTP GET request and get the response body as bytes
    let response = reqwest::get("http://en.wikipedia.org/wiki/Comparison_of_text_editors")
        .await?
        .text()
        .await?;

    // Parse the HTML response body using the scraper crate
    let document = Html::parse_document(&response);

    // Define a CSS selector to target the main comparison table
    let table_selector = Selector::parse("table.wikitable")?;

    // Find the main comparison table in the HTML document
    if let Some(table) = document.select(&table_selector).nth(1) {
        // Create a CSV file for writing
        let mut csv_writer = WriterBuilder::new()
            .has_headers(true)
            .from_path("editors.csv")?;

        // Iterate through table rows
        for row in table.select(&Selector::parse("tr")?) {
            let mut csv_row = Vec::new();

            // Iterate through cells in the row
            for cell in row.select(&Selector::parse("td, th")?) {
                csv_row.push(cell.text().collect::<String>().trim().to_string());
            }

            // Write the CSV row
            csv_writer.write_record(csv_row)?;
        }

        csv_writer.flush()?;
        println!("CSV file 'editors.csv' written successfully!");
    } else {
        println!("Main comparison table not found.");
    }

    Ok(())
}
