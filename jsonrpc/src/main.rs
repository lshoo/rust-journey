use jsonrpc::start_server;

fn main() {
    println!("Hello, world!");
    let res = start_server();
    res.unwrap();
}
