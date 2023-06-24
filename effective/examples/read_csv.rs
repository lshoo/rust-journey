fn main() {
    let filename = "data/data-example.csv";
    let has_headers = true;
    let delimiter = b',';

    match effective::read_csv::read_csv_file(filename, has_headers, delimiter) {
        Ok(rd) => {
            for r in &rd {
                for idx in 0..r.len() {
                    println!("{:?}", r.get(idx));
                }
                println!("{:?}", r);
            }
            println!("\nTotal rows: {}", &rd.len());
            println!("Done");
        }
        Err(e) => println!("Error: {}", e),
    }
}
