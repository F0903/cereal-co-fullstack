use super::api_response::{ApiResponse, Error};

pub type ApiResult<T> = Result<ApiResponse<T>, ApiResponse<Error>>;

impl<T> From<ApiResponse<T>> for ApiResult<T> {
    fn from(value: ApiResponse<T>) -> Self {
        Self::Ok(value)
    }
}
