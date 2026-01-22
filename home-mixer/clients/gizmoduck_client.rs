use std::collections::HashMap;

pub struct UserProfile {
    pub screen_name: String,
}

pub struct UserCounts {
    pub followers_count: i32,
}

pub struct UserInner {
    pub profile: UserProfile,
    pub counts: UserCounts,
}

pub struct User {
    pub user: Option<UserInner>,
    pub id: i64,
}

#[tonic::async_trait]
pub trait GizmoduckClient: Send + Sync {
    async fn get_users(&self, _: Vec<i64>) -> Result<HashMap<i64, Option<User>>, String>;
}

pub struct ProdGizmoduckClient;

#[tonic::async_trait]
impl GizmoduckClient for ProdGizmoduckClient {
    async fn get_users(&self, _: Vec<i64>) -> Result<HashMap<i64, Option<User>>, String> { Ok(HashMap::new()) }
}

impl ProdGizmoduckClient {
    pub async fn new() -> Result<Self, String> { Ok(Self) }
}
