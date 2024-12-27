use axum::routing::{delete, get, patch, post, Router};

use crate::{
    app_state::AppState,
    handlers::{
        application_root::{gateway_health_check, global_not_found_handler, grpc_health_check},
        authentication::{forgotten_password, login, set_new_password, sign_up},
    },
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Bookmark gRPC->HTTP gateway" }))
        .nest("/v1/health", {
            Router::new()
                .route("/", get(gateway_health_check))
                .route("/grpc", get(grpc_health_check))
        })
        .nest("/v1/auth", {
            Router::new()
                .route("/sign-up", post(sign_up))
                .route("/login", post(login))
                .route("/password-reset", post(forgotten_password))
                .route("/set-new-password", post(set_new_password))
        })
        .fallback(global_not_found_handler)
}
