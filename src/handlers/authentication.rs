use axum::Json;
use bookmark_grpc_codegen::client_stub::authentication::{
    authentication_client::AuthenticationClient, ForgottenPasswordRequest,
    ForgottenPasswordResponse, LoginRequest, LoginResponse, SetNewPasswordRequest,
    SetNewPasswordResponse, SignUpRequest, SignUpResponse,
};
use tracing::trace;

use crate::{
    api_response::{ApiResponse, ApiResponseJson, FromJson},
    config::CONFIG,
    error::AppError,
};

#[axum::debug_handler]
pub async fn sign_up(Json(payload): Json<SignUpRequest>) -> ApiResponseJson<SignUpResponse> {
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

    let mut grpc_client = AuthenticationClient::new(channel);

    let response = grpc_client
        .sign_up(payload)
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

pub async fn login(Json(payload): Json<LoginRequest>) -> ApiResponseJson<LoginResponse> {
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

    let mut grpc_client = AuthenticationClient::new(channel);

    let response = grpc_client.login(payload).await.map_err(|error_message| {
        trace!(
            "couldn't not process request due to error  Err: {}",
            error_message
        );
        AppError::ServerError { message: None }
    })?;
    let (_, body, _) = response.into_parts();

    Ok(ApiResponse::from_json(body))
}

pub async fn forgotten_password(
    Json(payload): Json<ForgottenPasswordRequest>,
) -> ApiResponseJson<ForgottenPasswordResponse> {
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

    let mut grpc_client = AuthenticationClient::new(channel);

    let response = grpc_client
        .forgotten_password(payload)
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

pub async fn set_new_password(
    Json(payload): Json<SetNewPasswordRequest>,
) -> ApiResponseJson<SetNewPasswordResponse> {
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

    let mut grpc_client = AuthenticationClient::new(channel);

    let response = grpc_client
        .set_new_password(payload)
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
