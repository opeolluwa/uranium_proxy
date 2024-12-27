use std::net::{Ipv4Addr, SocketAddrV4};

use crate::app_state::AppState;
use crate::config::CONFIG;
use anyhow::Ok;
use routes::router;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tower_http::validate_request::ValidateRequestHeaderLayer;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub mod api_response;
pub mod app_state;
pub mod config;
pub mod error;
pub mod handlers;
pub mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_line_number(true)
        .with_thread_ids(true)
        .with_max_level(Level::DEBUG)
        .pretty()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Error setting global tracing subscriber");

    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, CONFIG.port);

    // cors, client certificate, validation
    let app_state = AppState::new();
    let app = router()
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_grpc()))
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new().allow_origin(Any).allow_methods(Any),
            // .allow_credentials(true),
        )
        .layer(ValidateRequestHeaderLayer::accept("application/json"))
        .layer(CompressionLayer::new())
        .with_state(app_state);
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("server running on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
