pub trait StratoClient: Send + Sync {
    fn get_user_features(&self, _: i64) -> crate::clients::strato_client::ProdStratoClientFuture {
        Box::pin(async { Ok(xai_strato::StratoValue { v: None, _marker: std::marker::PhantomData }) })
    }
    fn store_request_info(&self, _: i64, _: Vec<i64>) -> crate::clients::strato_client::ProdStratoClientFuture {
        Box::pin(async { Ok(xai_strato::StratoValue { v: None, _marker: std::marker::PhantomData }) })
    }
}
pub type ProdStratoClientFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Result<xai_strato::StratoValue<Vec<u8>>, String>> + Send>>;

#[derive(Clone)]
pub struct ProdStratoClient;
impl StratoClient for ProdStratoClient {}
impl ProdStratoClient {
    pub async fn new() -> Result<Self, String> { Ok(Self) }
    pub async fn get_user_features(&self, _: i64) -> Result<xai_strato::StratoValue<Vec<u8>>, String> { Ok(xai_strato::StratoValue { v: None, _marker: std::marker::PhantomData }) }
}
