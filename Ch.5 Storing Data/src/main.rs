use csv::Writer;
use std::error::Error;
use std::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let csv_file_path = "test.csv";

    let csv_file = File::create(csv_file_path)?;
    let mut csv_writer = Writer::from_writer(csv_file);

    csv_writer.write_record(["number", "number plus 2", "number times 2"])?;

    for i in 0..10 {
        csv_writer.write_record([i.to_string(), (i + 2).to_string(), (i * 2).to_string()])?;
    }

    csv_writer.flush()?;

    println!("CSV file written successfully.");

    Ok(())
}
