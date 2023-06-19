use effective::get_files::get_files_from_folder;

fn main() {
    // match get_files_from_directory("/tmp") {
    //     Ok(file_names) => {

    //         for file_name in file_names.as_slice() {
    //             println!("{}", file_name);
    //         }

    //         // Println filenames size
    //         println!("{}", file_names.len());
    //     }
    //     Err(e) => println!("Error: {}", e),
    // }
    match get_files_from_folder("/tmp") {
        Ok(files) => {
            for file in files {
                if file.is_dir() {
                    println!("{} is a directory", file.display());
                    continue;
                }
                if file.is_symlink() {
                    println!("{} is a symlink", file.display());
                    continue;
                }

                let Ok(m) = file.metadata() else {
                    println!("Could not get metadata for {}", file.display());
                    continue;
                };

                if m.len() == 0 {
                    println!("{} is an empty file", file.display());
                    continue;
                }
                println!("{} is a file", file.display());
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
