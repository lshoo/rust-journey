use std::net::TcpListener;

use effective::tcp_server::handle;

fn main() -> std::io::Result<()> {
    // Create a tcp listener
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // Accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle each connection
                std::thread::spawn(move || handle(stream));
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
