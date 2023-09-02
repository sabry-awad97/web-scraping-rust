use std::io::{self, Read};
use zip::ZipArchive;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch the Word document
    let url = "http://pythonscraping.com/pages/AWordDocument.docx";
    let response = reqwest::get(url).await?;
    let word_file = response.bytes().await?;

    // Wrap the Word file in a Read cursor
    let cursor = io::Cursor::new(word_file);

    // Open the Word document as a Zip archive
    let mut archive = ZipArchive::new(cursor)?;

    // Read the content of 'word/document.xml'
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        if entry.name() == "word/document.xml" {
            let mut xml_content = String::new();
            entry.read_to_string(&mut xml_content)?;
            println!("{}", xml_content);
            break;
        }
    }

    Ok(())
}
