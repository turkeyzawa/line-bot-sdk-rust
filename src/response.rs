pub trait APIResponse: std::fmt::Debug {}

#[derive(Debug, Clone)]
pub struct MessageAPIResponse {
    request_id: Option<String>,
}

impl APIResponse for MessageAPIResponse {}

impl MessageAPIResponse {
    pub fn new(request_id: Option<String>) -> Self {
        Self { request_id }
    }
}
