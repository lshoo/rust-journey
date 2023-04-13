pub mod server;

use std::net::SocketAddr;

use anyhow::Result;
use jsonrpsee::server::ServerBuilder;
use log::info;
use server::{RoochServer, RpcServiceServer};
use tokio::signal::ctrl_c;
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
pub async fn start_server() -> Result<()> {
    let server_url: SocketAddr = "127.0.0.1:8080".parse()?;
    let rpc_service = RoochServer::default();
    let server = ServerBuilder::default().build(&server_url).await?;

    let handle = server.start(rpc_service.into_rpc())?;

    info!("starting listening {}", server_url);
    let mut sig_int = signal(SignalKind::interrupt()).unwrap();
    let mut sig_term = signal(SignalKind::terminate()).unwrap();

    tokio::select! {
        _ = sig_int.recv() => info!("receive SIGINT"),
        _ = sig_term.recv() => info!("receive SIGTERM"),
        _ = ctrl_c() => info!("receive Ctrl C"),
    }

    handle.stop().unwrap();
    info!("Shutdown program");

    Ok(())
}
