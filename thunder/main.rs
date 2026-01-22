use anyhow::Result;
use clap::Parser;
use log::info;
use std::sync::Arc;
use tonic::transport::Server;

use xai_thunder::{
    args, 
    // kafka_utils, 
    posts::post_store::PostStore, 
    strato_client::StratoClient,
    thunder_service::ThunderServiceImpl,
};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = args::Args::parse();

    // Initialize PostStore
    let post_store = Arc::new(PostStore::new(
        args.post_retention_seconds,
        args.request_timeout_ms,
    ));
    info!(
        "Initialized PostStore for in-memory post storage (retention: {} seconds, request_timeout: {}ms)",
        args.post_retention_seconds,
        args.request_timeout_ms
    );

    // Initialize StratoClient for fetching following lists
    let strato_client = Arc::new(StratoClient::new());
    info!("Initialized StratoClient");
    
    // Create ThunderService
    let thunder_service = ThunderServiceImpl::new(
        Arc::clone(&post_store),
        Arc::clone(&strato_client),
        args.max_concurrent_requests,
    );

    let addr = format!("0.0.0.0:{}", args.grpc_port).parse()?;
    info!("Server listening on {}", addr);

    Server::builder()
        .add_service(thunder_service.server())
        .serve(addr)
        .await?;

    Ok(())
}
