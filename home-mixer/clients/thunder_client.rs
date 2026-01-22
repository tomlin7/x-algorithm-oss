pub struct ThunderClient;
#[derive(Clone, Copy)]
pub enum ThunderCluster { Amp }
impl ThunderClient {
    pub async fn new() -> Self { Self }
    pub fn get_random_channel(&self, _: ThunderCluster) -> Option<tonic::transport::Channel> { None }
}
