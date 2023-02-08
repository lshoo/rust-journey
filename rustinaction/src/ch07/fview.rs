use std::{env, fs::File, io::prelude::*};

const BYTES_PER_LINE: usize = 16;
const INPUT: &'static [u8] = br#"
fn main() {
    println!("Hello RustinAction");
}"#;

pub fn run_str() -> std::io::Result<()> {
    let mut buffer: Vec<u8> = vec![];
    INPUT.read_to_end(&mut buffer)?;

    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        print!("[0x{position_in_input:08x}] ");
        for byte in line {
            print!("{byte:02x} ");
        }

        println!();
        position_in_input += BYTES_PER_LINE;
    }

    Ok(())
}

pub fn run() -> std::io::Result<()> {
    let arg1 = env::args().nth(1);
    let filename = arg1.expect("usage: fview FILENAME");

    let mut f = File::open(&filename).expect(&format!("Unable to open file: {filename:?}"));
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    while f.read_exact(&mut buffer).is_ok() {
        print!("[0x{pos:08x}");
        for byte in &buffer {
            match *byte {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{byte:02x}"),
            }
        }

        println!();

        pos += BYTES_PER_LINE;
    }

    Ok(())
}
