use std::{
    fs,
    io::{copy, Cursor},
};

use anyhow::Result;

pub async fn download_file(url: &str, filename: &str) -> Result<()> {
    // Send an http reqest to the URL
    let response = reqwest::get(url).await?;
    // Create a new file to write the downloaded image to
    let mut file = fs::File::create(filename)?;

    // create a cursor that wraps the response body
    let mut content = Cursor::new(response.bytes().await?);
    // Copy the content from the cursor to the file
    copy(&mut content, &mut file)?;

    Ok(())
}
