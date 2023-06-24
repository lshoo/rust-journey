use anyhow::Result;
use csv::StringRecord;

pub fn read_csv_file(
    filename: &str,
    has_headers: bool,
    delimiter: u8,
) -> Result<Vec<StringRecord>> {
    // Open the csv file
    let file = std::fs::File::open(filename)?;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(has_headers)
        .delimiter(delimiter)
        .from_reader(file);

    Ok(rdr.records().filter_map(|r| r.ok()).collect())
}
