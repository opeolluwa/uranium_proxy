use axum::{http::StatusCode, Json};

use crate::error::AppError;

/// ApiResponse
///
/// The ApiResponse<T> is an Abstraction of a Result type,
/// the Ok arm matches to (axum::http::StatusCode, T), where T is the generic type in the Wrapper,
///
///
/// ## Example
/// ```rust
/// pub async fn do_something()-> ApiResponse<String>{
/// .
/// .
/// .
/// Ok(ApiResponse::from_parts(String::from("hehehe"), StatusCode::Ok))
/// }
/// ```

pub type ApiResponse<T> = Result<(StatusCode, T), AppError>;

pub type ApiResponseJson<T> = Result<(StatusCode, Json<T>), AppError>;

pub type ResponseBody<T> = Json<T>;

pub trait FromParts<T> {
    fn from_parts(data: T, status: StatusCode) -> (StatusCode, T);
}
pub trait FromData<T> {
    fn from_data(data: T) -> (StatusCode, T);
}

pub trait FromJson<T> {
    fn from_json(data: T) -> (StatusCode, Json<T>);
}

impl<T> FromParts<T> for ApiResponse<T> {
    fn from_parts(data: T, status_code: StatusCode) -> (StatusCode, T) {
        (status_code, data)
    }
}

impl<T> FromData<T> for ApiResponse<T> {
    fn from_data(data: T) -> (StatusCode, T) {
        (StatusCode::OK, data)
    }
}

impl<T> FromJson<T> for ApiResponse<Json<T>> {
    fn from_json(data: T) -> (StatusCode, Json<T>) {
        (StatusCode::OK, Json(data))
    }
}
