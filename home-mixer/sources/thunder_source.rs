use crate::candidate_pipeline::candidate::PostCandidate;
use crate::candidate_pipeline::query::ScoredPostsQuery;
use crate::clients::thunder_client::ThunderClient;
use std::sync::Arc;
use tonic::async_trait;
use xai_candidate_pipeline::source::Source;
use xai_home_mixer_proto as pb;

pub struct ThunderSource {
    pub thunder_client: Arc<ThunderClient>,
}

#[async_trait]
impl Source<ScoredPostsQuery, PostCandidate> for ThunderSource {
    #[xai_stats_macro::receive_stats]
    async fn get_candidates(&self, query: &ScoredPostsQuery) -> Result<Vec<PostCandidate>, String> {
        let mut candidates = Vec::with_capacity(10_000);
        let start_id = 1_000_000; // distinct from other IDs
        
        for i in 0..10_000 {
            let tweet_id = (start_id + i) as i64;
            let author_id = (100 + (i % 50)) as u64; // 50 authors
            
            candidates.push(PostCandidate {
                tweet_id,
                author_id,
                served_type: Some(pb::ServedType::ForYouInNetwork),
                ..Default::default()
            });
        }
        
        Ok(candidates)
    }
}
