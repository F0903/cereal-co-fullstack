use super::api_response::{ApiError, ApiResponse};

pub type ApiResult<T> = Result<ApiResponse<T>, ApiResponse<ApiError>>;

pub trait ApiResultIntoOk<T> {
    fn into_ok(self) -> ApiResult<T>;
}

pub trait ApiResultIntoError<T> {
    fn into_error(self) -> ApiResult<T>;
}

impl<T> ApiResultIntoOk<T> for ApiResponse<T> {
    fn into_ok(self) -> ApiResult<T> {
        ApiResult::Ok(self)
    }
}

impl<T> ApiResultIntoError<T> for ApiResponse<ApiError> {
    fn into_error(self) -> ApiResult<T> {
        ApiResult::Err(self)
    }
}
