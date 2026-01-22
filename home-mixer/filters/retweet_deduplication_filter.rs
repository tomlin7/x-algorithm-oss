use crate::candidate_pipeline::candidate::PostCandidate;
use crate::candidate_pipeline::query::ScoredPostsQuery;
use std::collections::HashSet;
use tonic::async_trait;
use xai_candidate_pipeline::filter::{Filter, FilterResult};

/// Deduplicates retweets, keeping only the first occurrence of a tweet
/// (whether as an original or as a retweet).
pub struct RetweetDeduplicationFilter;

#[async_trait]
impl Filter<ScoredPostsQuery, PostCandidate> for RetweetDeduplicationFilter {
    async fn filter(
        &self,
        _query: &ScoredPostsQuery,
        candidates: Vec<PostCandidate>,
    ) -> Result<FilterResult<PostCandidate>, String> {
        let mut seen_tweet_ids: HashSet<u64> = HashSet::new();
        let mut kept = Vec::new();
        let mut removed = Vec::new();

        for candidate in candidates {
            match candidate.retweeted_tweet_id {
                Some(retweeted_id) => {
                    // Remove if we've already seen this tweet (as original or retweet)
                    if seen_tweet_ids.insert(retweeted_id) {
                        kept.push(candidate);
                    } else {
                        removed.push(candidate);
                    }
                }
                None => {
                    // Mark this original tweet ID as seen so retweets of it get filtered
                    if seen_tweet_ids.insert(candidate.tweet_id as u64) {
                        kept.push(candidate);
                    } else {
                        removed.push(candidate);
                    }
                }
            }
        }

        Ok(FilterResult { kept, removed })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::candidate_pipeline::candidate::PostCandidate;

    #[tokio::test]
    async fn test_retweet_deduplication_logic() {
        let filter = RetweetDeduplicationFilter;
        let query = ScoredPostsQuery::default();
        
        let mut candidates = vec![
            // Retweet of 10 comes first
            PostCandidate { 
                tweet_id: 100, 
                retweeted_tweet_id: Some(10), 
                ..Default::default() 
            },
            // Original 10 comes second
            PostCandidate { 
                tweet_id: 10, 
                retweeted_tweet_id: None, 
                ..Default::default() 
            },
        ];
        
        let result = filter.filter(&query, candidates).await.unwrap();
        
        // Expected behavior:
        // 1. 100 is processed. retweeted_id=10. seen(10) -> true. Keep 100.
        // 2. 10 is processed. tweet_id=10. seen(10) -> false. Remove 10.
        // Result: Keep 100 (the retweet).
        
        assert_eq!(result.kept.len(), 1, "Should keep exactly one candidate");
        assert_eq!(result.kept[0].tweet_id, 100, "Should keep the retweet (first occurrence of content)");
        assert_eq!(result.removed.len(), 1, "Should remove the original tweet as duplicate");
        assert_eq!(result.removed[0].tweet_id, 10, "Should remove tweet 10");
    }
}
