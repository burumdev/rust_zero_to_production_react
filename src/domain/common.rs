use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
}
