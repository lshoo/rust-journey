pub mod binary_search;

pub mod download_file;

pub mod env;

pub mod enum_str;

pub mod get_files;

pub mod hash_file;

pub mod read_csv;

pub mod read_image_dimension;

pub mod tcp_server;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
