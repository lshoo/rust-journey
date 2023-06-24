use effective::binary_search::binary_search;

fn main() {
    let data = vec![1, 2, 2, 3, 4, 5, 6, 8, 20];
    match binary_search(&data, &2) {
        Some(i) => println!("Found at index {}", i),
        None => println!("Not found"),
    }
}
