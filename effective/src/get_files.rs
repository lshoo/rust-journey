use std::{fs, io, path::PathBuf};

pub fn get_files_from_directory(path: &str) -> io::Result<Vec<String>> {
    // Get a list of all entries in the folder
    let entries = fs::read_dir(path)?;

    // Extract the filenames from the directory entries and store them in a vector
    let files = entries
        // .filter_map(|entry| entry.ok().map(|e| {
        //     e.path()
        //         .file_name()?
        //         .to_str()
        //         .map(|s| s.to_owned())
        // }).unwrap())
        // .collect::<Vec<_>>();
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() {
                path.file_name()?.to_str().map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect();

    Ok(files)
}

pub fn get_files_from_folder(path: &str) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?;
    let all: Vec<_> = entries
        .filter_map(|entry| Some(entry.ok()?.path()))
        .collect();

    Ok(all)
}
