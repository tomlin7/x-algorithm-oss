use xai_recsys_proto::{TweetInfo, UserActionSequence};

pub struct ScoredCandidate {
    pub candidate: Option<TweetInfo>,
    pub score: f64,
}

pub struct ScoredCandidates {
    pub candidates: Vec<ScoredCandidate>,
}

pub struct PhoenixRetrievalResponse {
    pub top_k_candidates: Vec<ScoredCandidates>,
}

#[tonic::async_trait]
pub trait PhoenixRetrievalClient: Send + Sync {
    async fn retrieve(&self, _: u64, _: UserActionSequence, _: i32) -> Result<PhoenixRetrievalResponse, String>;
}
pub struct ProdPhoenixRetrievalClient;
#[tonic::async_trait]
impl PhoenixRetrievalClient for ProdPhoenixRetrievalClient {
    async fn retrieve(&self, _: u64, _: UserActionSequence, _: i32) -> Result<PhoenixRetrievalResponse, String> { 
        Ok(PhoenixRetrievalResponse { top_k_candidates: vec![] }) 
    }
}
impl ProdPhoenixRetrievalClient {
    pub async fn new() -> Result<Self, String> { Ok(Self) }
}
