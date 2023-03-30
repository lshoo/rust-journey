#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    concurrencies::ractor_ping::run().await;
}
