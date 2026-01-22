pub mod bloom_filter;
pub mod candidates_util;
pub mod score_normalizer;
pub mod snowflake;
pub mod request_util {
    pub fn generate_request_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }
}
