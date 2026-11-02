use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<Value>,
}
