use hyper::Uri;
use tonic::transport::Endpoint;

lazy_static::lazy_static! {
  pub static ref  CONFIG : Config  = Config::parse();
}

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub grpc_base_url: Endpoint,
    // pub message_queue_channel: String,
}

impl Config {
    pub fn parse() -> Self {
        let port = std::env::var("HTTP_SERVICE_PORT")
            .expect("Couldn't HTTP_SERVICE_PORT env")
            .parse::<u16>()
            .expect("HTTP_SERVICE_PORT is not a number");

        let grpc_base_url = std::env::var("GRPC_BASE_URL")
            .expect("GRPC_BASE_URL not provided ")
            .parse::<Uri>()
            .expect("Invalid URI for GRPC_BASE_URL env");

        Self {
            port,
            grpc_base_url: Endpoint::from(grpc_base_url),
        }
    }
}
