use super::api_response::{ApiError, ApiResponse};

pub type ApiResult<T> = Result<ApiResponse<T>, ApiResponse<ApiError>>;

impl<T> From<ApiResponse<T>> for ApiResult<T> {
    fn from(value: ApiResponse<T>) -> Self {
        Self::Ok(value)
    }
}
