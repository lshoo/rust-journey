use effective::hash_file::hash_file;

fn main() {
    let file = "Cargo.toml";
    match hash_file(file) {
        Ok(hash) => println!("Hash of {} is {}", file, hash),
        Err(e) => println!("Error is {}", e),
    }
}
