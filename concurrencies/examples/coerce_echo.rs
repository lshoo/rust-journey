#[tokio::main]
async fn main() {
    concurrencies::coerces::coerce_timer::run().await;
}
