use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

fn main() {
    write_numbers_to_file();
    read_numbers_from_file();
}

fn write_numbers_to_file() -> (u32, i8, f64) {
    let mut w = vec![];

    let one: u32 = 123;
    let two: i8 = 123;
    let three: f64 = 123.9;

    w.write_u32::<LittleEndian>(one).unwrap();
    println!("{w:?}");

    w.write_i8(two).unwrap();
    println!("{w:?}");

    w.write_f64::<LittleEndian>(three).unwrap();
    println!("{w:?}");

    (one, two, three)
}

fn read_numbers_from_file() -> (u32, i8, f64) {
    let mut r = Cursor::new(vec![1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 8, 64]);
    let one = r.read_u32::<LittleEndian>().unwrap();
    let two = r.read_i8().unwrap();
    let three = r.read_f64::<LittleEndian>().unwrap();

    println!("one: {one}, two: {two}, three: {three}");

    (one, two, three)
}
