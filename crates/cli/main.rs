#![forbid(unsafe_code)]

use std::net::{Ipv4Addr, SocketAddr};

use clap::Parser;
use tonic::transport::Server;

use crate::handler::{InstanceService, RegistryService};
use crate::service::{AppConfig, AppState};

mod handler;
mod middleware;
mod service;

/// Command-line arguments.
#[derive(Debug, Parser)]
pub struct Args {
    /// Bound server port.
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Service.
    let config = AppConfig::builder().build();
    let state = AppState::new(config);

    let instance = InstanceService::new(state.clone());
    let instance = instance.into_server();

    let registry = RegistryService::new(state);
    let registry = registry.into_server();

    // Listen.
    let server_addr = SocketAddr::from((Ipv4Addr::LOCALHOST, args.port));
    tracing::debug!(
        target: "server:setup", port = args.port,
        "server is listening on {}", server_addr,
    );

    Server::builder()
        .add_service(instance)
        .add_service(registry)
        .serve(server_addr)
        .await?;

    Ok(())
}
