# Storing Scraped Data

## Importance of Data Storage

After successfully scraping data from websites, it's important to store the collected information in an organized and accessible manner. Proper data storage ensures that you can analyze, process, and utilize the scraped data effectively.

### Choosing the Right Storage Approach

The choice of storage method depends on factors such as the type of data, its volume, and your project's requirements. Here are some common storage approaches:

1. **Database:** Databases like MySQL, PostgreSQL, or MongoDB are suitable for structured data. They offer efficient querying and indexing capabilities.
2. **CSV or JSON Files:** For smaller datasets, you can save data in CSV (Comma-Separated Values) or JSON (JavaScript Object Notation) format. These formats are human-readable and can be easily imported into various applications.
3. **Data Warehouses:** For large-scale projects, data warehouses like Amazon Redshift or Google BigQuery provide scalable storage and advanced analytics capabilities.
4. **Cloud Storage:** Store data in cloud-based services like Amazon S3, Google Cloud Storage, or Microsoft Azure Blob Storage. This ensures data accessibility and scalability.

### Structuring the Data

When storing scraped data, consider structuring it in a way that mirrors the original website's structure. For example, if you're scraping product information from an e-commerce site, you might have tables or collections for products, categories, and reviews.

### Data Cleaning and Transformation

Before storing data, it's often necessary to perform data cleaning and transformation. This includes handling missing values, removing duplicates, and converting data into a consistent format.

### Backup and Security

Data is valuable, so ensure you have proper backup mechanisms in place. Additionally, consider data security and privacy concerns, especially if the scraped data contains sensitive information.

### Example

If you're scraping weather data from various cities, you might structure your database with a "Cities" table and a "WeatherData" table. The "WeatherData" table could have columns for date, temperature, humidity, etc., and a foreign key referencing the city from the "Cities" table.

## Handling Media Files

### Understanding Media Files

Media files, such as images, videos, audio files, and documents, are commonly found on websites. Web scraping involving media files requires special consideration and techniques to ensure you can efficiently and ethically retrieve and manage such content.

### Techniques for Scraping Media Files

1. **Image URLs**: In many cases, you won't need to download the actual media files. You can often extract URLs to the media files and store them for future reference.

1. **Downloading Media**: If you do need to download media files, be sure to respect the website's terms of use and copyright. Download only the media files you have permission to use.

1. **Media Scraping Libraries**: Some programming languages offer libraries specifically designed for media scraping. For example, Rust's `reqwest` library can download media files, and libraries like `scraper` can extract media URLs.

1. **Asynchronous Downloading**: To speed up the downloading process, consider using asynchronous programming techniques. This allows you to download multiple media files concurrently.

### Storing and Managing Media Files

When dealing with media files, it's important to have a clear organizational structure:

- **File Naming**: Choose descriptive and unique filenames for downloaded media files. You might use a combination of the file's original name, an identifier, and a timestamp.

- **File Storage**: Store media files in an appropriate location, such as a designated folder or a cloud storage service.

### Ethical Considerations

When scraping media files, always respect the website's terms of use, copyright laws, and intellectual property rights. Do not scrape or download media files that you do not have the legal right to use.

```rs
use scraper::{Html, Selector};
use std::fs::File;
use std::io::Write;

#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("File operation failed: {0}")]
    FileError(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let url = "http://www.pythonscraping.com";
    let image_title = "logo01";
    let image_file_name = "logo.jpg";

    let html = fetch_html(url).await?;
    let image_location = find_image_location(&html, image_title);
    if let Some(image_url) = image_location {
        download_image(image_url, image_file_name).await?;
        println!("Image downloaded successfully.");
    } else {
        println!("Image not found on the page.");
    }

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, AppError> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn find_image_location(html: &str, title: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(&format!("img[title='{}']", title)).unwrap();

    if let Some(image_element) = document.select(&selector).next() {
        image_element.value().attr("src").map(String::from)
    } else {
        None
    }
}

async fn download_image(image_url: String, file_name: &str) -> Result<(), AppError> {
    let response = reqwest::get(&image_url).await?;
    let mut image_file = File::create(file_name)?;
    let image_bytes = response.bytes().await?;
    image_file.write_all(&image_bytes)?;
    Ok(())
}
```

```rs
use scraper::{Html, Selector};
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("File operation failed: {0}")]
    FileError(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let base_url = "http://pythonscraping.com";
    let download_directory = "downloaded";

    let html = fetch_html(base_url).await?;
    let download_list = find_download_list(&html);

    for download in download_list {
        if let Some(file_url) = get_absolute_url(base_url, &download) {
            println!("{}", file_url);
            let download_path = get_download_path(&file_url, download_directory)?;
            download_file(&file_url, &download_path).await?;
            println!("Downloaded: {}", download_path);
        }
    }

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, AppError> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn find_download_list(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("[src]").unwrap();

    document
        .select(&selector)
        .filter_map(|element| element.value().attr("src"))
        .map(String::from)
        .collect()
}

fn get_absolute_url(base_url: &str, source: &str) -> Option<String> {
    if source.starts_with("http://") || source.starts_with("https://") {
        Some(source.to_string())
    } else {
        Some(format!("{}/{}", base_url, source))
    }
}

fn sanitize_filename(filename: &str) -> String {
    filename.replace(['/', ':', '?'], "_")
}

fn get_download_path(file_url: &str, download_directory: &str) -> Result<String, AppError> {
    let filename = file_url
        .rsplit('/')
        .next()
        .unwrap_or("unknown_file")
        .split('?')
        .next()
        .expect("Invalid file URL");

    let sanitized_filename = sanitize_filename(filename);
    let full_path = format!("{}/{}", download_directory, sanitized_filename);
    let directory = Path::new(&full_path).parent().ok_or_else(|| {
        AppError::FileError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Parent directory not found",
        ))
    })?;

    if !directory.exists() {
        fs::create_dir_all(directory)?;
    }

    Ok(full_path.to_string())
}

async fn download_file(file_url: &str, file_path: &str) -> Result<(), AppError> {
    let response = reqwest::get(file_url).await?;
    let mut file = fs::File::create(file_path)?;
    let bytes = response.bytes().await?;
    file.write_all(&bytes)?;
    Ok(())
}
```

## Storing Data in CSV Format

Storing scraped data in CSV (Comma-Separated Values) format is a simple and widely used method. CSV files are human-readable and can be easily opened in spreadsheet applications like Microsoft Excel or Google Sheets. Let's explore how to store scraped data in CSV format.

### Writing Data to CSV in Rust

Rust provides crates that make working with CSV files straightforward. Here's an example of how to store scraped data to a CSV file using the `csv` crate:

```rs
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
```

### Benefits of CSV Storage

- **Simplicity:** CSV is easy to work with and doesn't require special software to open.
- **Compatibility:** CSV files can be imported into various applications, databases, and programming languages.
- **Human-Readable:** The data is stored in plain text, making it easy to understand and debug.

### Considerations

- **Data Format:** Ensure that your data is properly formatted before writing to CSV. Strings and numbers should be in the correct format.
- **Header Row:** Including a header row with column names improves the readability of the CSV file.
- **Special Characters:** Be cautious with special characters like commas and line breaks in the data, as they might affect CSV parsing.

## Storing data in MySQL

MySQL is a popular open-source relational database management system that is commonly used for storing and managing structured data. When combined with the Rust programming language, it can be a powerful tool for web scraping and data manipulation

### Integrating MySQL with Rust

```rs
use std::env;

use dotenvy::dotenv;
use mysql::prelude::*;

fn main() -> Result<(), mysql::Error> {
    dotenv().ok();

    let db_user = env::var("DB_USER").expect("DB_USER not set");
    let db_pass = env::var("DB_PASS").expect("DB_PASS not set");

    let mut opts = mysql::OptsBuilder::new();
    opts = opts.user(Some(db_user)).pass(Some(db_pass));

    let pool = mysql::Pool::new(opts)?;

    let mut conn = pool.get_conn()?;

    conn.query_drop("DROP DATABASE IF EXISTS scraping")?;
    conn.query_drop("CREATE DATABASE scraping")?;
    conn.query_drop("USE scraping")?;

    conn.query_drop(
        r#"
        CREATE TABLE pages (
            id BIGINT NOT NULL AUTO_INCREMENT,
            title VARCHAR(200),
            content VARCHAR(10000),
            created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY(id)
        )
    "#,
    )?;

    Ok(())
}
```
