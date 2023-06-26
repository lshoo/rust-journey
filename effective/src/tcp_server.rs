use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle(mut stream: TcpStream) {
    // 64KB buffer for demonstration purposes
    let mut buffer = [0; 65536];

    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => return, // End of stream
            Ok(n) => n,
            Err(_) => return,
        };

        let msg = String::from_utf8_lossy(&buffer[..bytes_read]);
        print!("received:{}.", msg);

        let cmd = msg.lines().next().map(|s| s.to_lowercase());
        if cmd == Some("q".to_owned()) || cmd == Some("quit".to_owned()) {
            return;
        }

        println!("sending:{}", msg);

        // Echo the data back to the client
        if stream.write(&buffer[..bytes_read]).is_err() {
            return; // Error writing to stream
        }
    }
}
