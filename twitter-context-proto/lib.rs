#[derive(Clone, Debug, Default)]
pub struct TwitterContextViewer {
    pub user_id: i64,
    pub client_application_id: i64,
    pub request_country_code: String,
    pub request_language_code: String,
}

pub trait GetTwitterContextViewer {
    fn get_viewer(&self) -> Option<TwitterContextViewer>;
}
