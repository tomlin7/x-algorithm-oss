use std::collections::HashMap;
use crate::candidate_pipeline::candidate_features::MediaInfo;

pub struct TweetCoreData {
    pub author_id: u64,
    pub source_user_id: Option<u64>,
    pub source_tweet_id: Option<u64>,
    pub in_reply_to_tweet_id: Option<u64>,
    pub text: String,
}

pub struct MediaEntity {
    pub media_info: Option<MediaInfo>,
}

#[tonic::async_trait]
pub trait TESClient: Send + Sync {
    async fn get_tweet_core_datas(&self, _: Vec<u64>) -> Result<HashMap<u64, TweetCoreData>, String>;
    async fn get_subscription_author_ids(&self, _: Vec<u64>) -> Result<HashMap<u64, u64>, String>;
    async fn get_tweet_media_entities(&self, _: Vec<u64>) -> Result<HashMap<u64, Vec<MediaEntity>>, String>;
}

pub struct ProdTESClient;

#[tonic::async_trait]
impl TESClient for ProdTESClient {
    async fn get_tweet_core_datas(&self, _: Vec<u64>) -> Result<HashMap<u64, TweetCoreData>, String> { Ok(HashMap::new()) }
    async fn get_subscription_author_ids(&self, _: Vec<u64>) -> Result<HashMap<u64, u64>, String> { Ok(HashMap::new()) }
    async fn get_tweet_media_entities(&self, _: Vec<u64>) -> Result<HashMap<u64, Vec<MediaEntity>>, String> { Ok(HashMap::new()) }
}

impl ProdTESClient {
    pub async fn new() -> Result<Self, String> { Ok(Self) }
}
