use anyhow::Result;
use sha2::Digest;
use std::io::Read;

pub fn hash_file(file: &str) -> Result<String> {
    // Open the file
    let mut file = std::fs::File::open(file)?;
    // Create a sha256 hasher
    let mut hasher = sha2::Sha256::new();
    // Read the file in 4KB chunks and feed them to the hasher
    let mut buffer = [0u8; 4096];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    // Finalize the hash and get the result as a hex string
    Ok(format!("{:x}", hasher.finalize()))
}
