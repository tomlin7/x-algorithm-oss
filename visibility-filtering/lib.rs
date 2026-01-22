pub mod models {
    #[derive(Clone, Debug)]
    pub struct SafetyResult {
        pub action: Action,
    }

    #[derive(Clone, Debug)]
    pub enum FilteredReason {
        SafetyResult(SafetyResult),
        Unspecified,
    }
    
    impl Default for FilteredReason {
        fn default() -> Self { Self::Unspecified }
    }
    
    impl From<FilteredReason> for i32 {
        fn from(_: FilteredReason) -> Self {
            0
        }
    }

    #[derive(Clone, Debug)]
    pub enum Action {
        Drop(String),
        Pass,
    }
}

pub mod vf_client {
    pub enum SafetyLevel {
        TimelineHome,
        TimelineHomeRecommendations,
    }
    
    #[tonic::async_trait]
    pub trait VisibilityFilteringClient: Send + Sync {
        async fn get_result(&self, _: Vec<i64>, _: SafetyLevel, _: i64, _: Option<xai_twittercontext_proto::TwitterContextViewer>) -> Result<std::collections::HashMap<i64, Option<super::models::FilteredReason>>, String>;
    }
    pub struct ProdVisibilityFilteringClient;
    #[tonic::async_trait]
    impl VisibilityFilteringClient for ProdVisibilityFilteringClient {
        async fn get_result(&self, _: Vec<i64>, _: SafetyLevel, _: i64, _: Option<xai_twittercontext_proto::TwitterContextViewer>) -> Result<std::collections::HashMap<i64, Option<super::models::FilteredReason>>, String> { Ok(std::collections::HashMap::new()) }
    }
    impl ProdVisibilityFilteringClient {
        pub async fn new(_: String, _: String, _: String) -> Result<Self, String> { Ok(Self) }
    }
}
