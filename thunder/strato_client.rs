#[derive(Clone)]
pub struct StratoClient;

impl StratoClient {
    pub fn new() -> Self {
        Self
    }
    pub async fn fetch_following_list(&self, _user_id: i64, _limit: i32) -> anyhow::Result<Vec<i64>> {
        Ok(vec![])
    }
}
