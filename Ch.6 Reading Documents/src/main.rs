use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::io::{self, Read};
use zip::ZipArchive;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch the Word document
    let docx_url = "http://pythonscraping.com/pages/AWordDocument.docx";
    let response = reqwest::get(docx_url).await?;
    let word_file = response.bytes().await?;

    // Wrap the Word file in a Read cursor
    let cursor = io::Cursor::new(word_file);

    // Open the Word document as a Zip archive
    let mut archive = ZipArchive::new(cursor)?;

    // Read the content of 'word/document.xml'
    let mut xml_content = String::new();

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        if entry.name() == "word/document.xml" {
            entry.read_to_string(&mut xml_content)?;
            break;
        }
    }

    let mut reader = Reader::from_str(&xml_content);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut inside_w_t = false;

    let mut extracted_text = String::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if e.name().as_ref() == b"w:t" {
                    inside_w_t = true;
                }
            }
            Ok(Event::Text(e)) => {
                if inside_w_t {
                    let text = &e.unescape()?;
                    extracted_text.push_str(text);
                }
            }
            Ok(Event::End(ref e)) => {
                if e.name().as_ref() == b"w:t" {
                    inside_w_t = false;
                }
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
        
        buf.clear();
    }

    println!("{}", extracted_text);

    Ok(())
}
