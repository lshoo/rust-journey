use std::time::Duration;

use clap::{command, Parser};
use reqwest::Client;
use serde_json::Value;
use thiserror::Error;

pub const COINGECKO_URL: &str = "http://api.coingecko.com/api/v3/simple/price";

#[derive(Error, Debug)]
pub enum CoingeckoError {
    #[error("Poll data timeout")]
    PollTimeout,
    #[error("The connection failed")]
    ConnectionFailed(#[from] reqwest::Error),
    // #[error("Serialize failed")]
    // SerializeFaild(#[from] serde_json::Error),
}

async fn get_price(chain: &str, currency: &str) -> Result<Value, CoingeckoError> {
    let client = reqwest::Client::builder().build()?;
    let params = [("ids", chain), ("vs_currencies", currency)];

    Ok(loop {
        async fn try_poll(client: &Client, params: &[(&str, &str)]) -> reqwest::Result<String> {
            client
                .get(COINGECKO_URL)
                .query(params)
                .send()
                .await?
                .text()
                .await
        }

        let response = tokio::time::timeout(Duration::from_secs(30), try_poll(&client, &params))
            .await
            .map_err(|_| CoingeckoError::PollTimeout)?
            .map_err(CoingeckoError::ConnectionFailed)?;

        let value: Result<Value, _> = serde_json::from_str(&response);
        println!("{value:?}");
        match value {
            Ok(v) => {
                let res = &v[chain][currency];
                break res.to_owned();
            }
            Err(_e) => tokio::time::sleep(Duration::from_secs(3)).await,
        }
    })
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    chain: String,

    #[arg(short, long)]
    vs: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let chain = args.chain;
    let currency = args.vs;

    match get_price(&chain, &currency).await {
        Ok(v) => {
            if let Some(v) = v.as_f64() {
                println!("{chain:?} vs {currency:?} = {v}");
            } else {
                println!("{v:?} is not a number");
            }
        }
        Err(e) => {
            println!("Error: {e:?}");
        }
    }
}
