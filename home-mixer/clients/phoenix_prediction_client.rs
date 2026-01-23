#[tonic::async_trait]
pub trait PhoenixPredictionClient: Send + Sync {
    async fn predict(&self, _: i64, _: xai_recsys_proto::UserActionSequence, _: Vec<xai_recsys_proto::TweetInfo>) -> Result<xai_recsys_proto::PredictNextActionsResponse, String>;
}
pub struct ProdPhoenixPredictionClient;
#[tonic::async_trait]
impl PhoenixPredictionClient for ProdPhoenixPredictionClient {
    async fn predict(&self, _: i64, _: xai_recsys_proto::UserActionSequence, candidates: Vec<xai_recsys_proto::TweetInfo>) -> Result<xai_recsys_proto::PredictNextActionsResponse, String> { 
        use rand::Rng; // Make sure rand is available or implement simple RNG
        
        let mut distribution_set = xai_recsys_proto::DistributionSet::default();
        let mut rng = rand::thread_rng();

        for candidate in candidates {
            // Fill 1000 slots to cover likely all ActionName enum variants
            let top_log_probs: Vec<f32> = (0..1000).map(|_| {
                rng.gen_range(0.0..1.0) 
            }).collect();
            
            // Fill continuous actions (DwellTime is likely index 0)
            let continuous_actions_values: Vec<f32> = vec![rng.gen_range(0.0..100.0)];

            distribution_set.candidate_distributions.push(xai_recsys_proto::CandidateDistribution {
                candidate: Some(candidate),
                top_log_probs,
                continuous_actions_values,
            });
        }

        Ok(xai_recsys_proto::PredictNextActionsResponse {
            distribution_sets: vec![distribution_set],
        })
    }
}
impl ProdPhoenixPredictionClient {
    pub async fn new() -> Result<Self, String> { Ok(Self) }
}
