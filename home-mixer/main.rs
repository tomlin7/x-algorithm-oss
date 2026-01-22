use clap::Parser;
use log::info;
use tonic::codec::CompressionEncoding;
use tonic::transport::Server;
use tonic_reflection::server::Builder;

use xai_home_mixer_proto as pb;
use xai_home_mixer::HomeMixerServer;
use xai_home_mixer::params;

#[derive(Parser, Debug)]
#[command(about = "HomeMixer gRPC Server")]
struct Args {
    #[arg(long)]
    grpc_port: u16,
    #[arg(long)]
    metrics_port: u16,
    #[arg(long)]
    reload_interval_minutes: u64,
    #[arg(long)]
    chunk_size: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    env_logger::init();
    
    info!(
        "Starting server with gRPC port: {}, metrics port: {}, reload interval: {} minutes, chunk size: {}",
        args.grpc_port, args.metrics_port, args.reload_interval_minutes, args.chunk_size,
    );

    let addr = format!("0.0.0.0:{}", args.grpc_port).parse()?;
    
    // Create the service implementation
    let service = HomeMixerServer::new().await;
    
    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(pb::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    info!("Server listening on {}", addr);

    Server::builder()
        .add_service(reflection_service)
        .add_service(
            pb::scored_posts_service_server::ScoredPostsServiceServer::new(service)
                .max_decoding_message_size(params::MAX_GRPC_MESSAGE_SIZE)
                .max_encoding_message_size(params::MAX_GRPC_MESSAGE_SIZE)
                .accept_compressed(CompressionEncoding::Gzip)
                .accept_compressed(CompressionEncoding::Zstd)
                .send_compressed(CompressionEncoding::Gzip)
                .send_compressed(CompressionEncoding::Zstd),
        )
        .serve(addr)
        .await?;

    Ok(())
}
