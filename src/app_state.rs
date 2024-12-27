use tonic::transport::Endpoint;

#[derive(Debug, Clone)]
pub struct AppState {
    pub grpc_channel: Endpoint,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            grpc_channel: Endpoint::from_static("http://plain-garland/bookmark-backend"),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
