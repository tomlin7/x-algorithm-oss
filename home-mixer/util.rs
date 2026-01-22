pub mod request_util {
    pub fn generate_request_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }
}
