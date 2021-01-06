use crate::api::cloudflare::api_error::ApiError;
use crate::api::cloudflare::api_result::ApiResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ApiResponse<T: ApiResult> {
    result: Option<Vec<T>>,
    errors: Vec<ApiError>,
}

impl<T: ApiResult> ApiResponse<T> {
    pub fn take_result(self) -> Option<Vec<T>> {
        self.result
    }

    pub fn errors(&self) -> &Vec<ApiError> {
        &self.errors
    }
}
