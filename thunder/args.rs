use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[arg(long, default_value = "86400")]
    pub post_retention_seconds: u64,
    
    #[arg(long, default_value = "1000")]
    pub request_timeout_ms: u64,
    
    #[arg(long, default_value = "100")]
    pub max_concurrent_requests: usize,
    
    #[arg(long, default_value = "8081")]
    pub grpc_port: u16,
    
    #[arg(long, default_value = "8080")]
    pub http_port: u16,
    
    #[arg(long, default_value = "false")]
    pub enable_profiling: bool,
    
    #[arg(long, default_value = "1")]
    pub kafka_num_threads: usize,
    
    #[arg(long, default_value = "true")]
    pub is_serving: bool,
}
