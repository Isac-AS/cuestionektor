use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum OperationResult {
    Success,
    Fail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackendResponse<T> {
    pub result: OperationResult,
    pub data: T,
}

impl<T> BackendResponse<T> {
    pub fn new(result: OperationResult, data: T) -> BackendResponse<T> {
        BackendResponse { result, data }
    }
}
