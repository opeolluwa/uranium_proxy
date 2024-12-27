use bookmark_grpc_codegen::client_stub::health_check::{
    health_check_client::HealthCheckClient, HealthCheckRequest, HealthCheckResponse,
};
use hyper::StatusCode;
use tonic::Request;
use tracing::trace;

use crate::{
    api_response::{ApiResponse, ApiResponseJson, FromData, FromJson, FromParts},
    config::CONFIG,
    error::AppError,
};

pub async fn gateway_health_check() -> ApiResponse<&'static str> {
    Ok(ApiResponse::from_data("The application is healthy"))
}

pub async fn grpc_health_check() -> ApiResponseJson<HealthCheckResponse> {
    let channel = CONFIG
        .grpc_base_url
        .connect()
        .await
        .map_err(|error_message| {
            trace!(
                "couldn't not connect to gRPC endpoint due to Err: {}",
                error_message
            );
            AppError::ServerError { message: None }
        })?;

    let mut health_check_grpc_client = HealthCheckClient::new(channel);

    let request = Request::new(HealthCheckRequest {});

    let response = health_check_grpc_client
        .check_service_health(request)
        .await
        .map_err(|error_message| {
            trace!(
                "couldn't not process request due to error  Err: {}",
                error_message
            );
            AppError::ServerError { message: None }
        })?;

    let (_, body, _) = response.into_parts();

    Ok(ApiResponse::from_json(body))
}

pub async fn global_not_found_handler() -> ApiResponse<&'static str> {
    Ok(ApiResponse::from_parts(
        "The resource you are looking for did not exist or has been permanently removed ",
        StatusCode::NOT_FOUND,
    ))
}
