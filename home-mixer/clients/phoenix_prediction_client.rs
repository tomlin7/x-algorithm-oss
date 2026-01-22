#[tonic::async_trait]
pub trait PhoenixPredictionClient: Send + Sync {
    async fn predict(&self, _: i64, _: xai_recsys_proto::UserActionSequence, _: Vec<xai_recsys_proto::TweetInfo>) -> Result<xai_recsys_proto::PredictNextActionsResponse, String>;
}
pub struct ProdPhoenixPredictionClient;
#[tonic::async_trait]
impl PhoenixPredictionClient for ProdPhoenixPredictionClient {
    async fn predict(&self, _: i64, _: xai_recsys_proto::UserActionSequence, _: Vec<xai_recsys_proto::TweetInfo>) -> Result<xai_recsys_proto::PredictNextActionsResponse, String> { Ok(xai_recsys_proto::PredictNextActionsResponse::default()) }
}
impl ProdPhoenixPredictionClient {
    pub async fn new() -> Result<Self, String> { Ok(Self) }
}
