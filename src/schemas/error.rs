use serde::Serialize;
use utoipa::ToSchema;

/// Standard error response returned by all failing endpoints
#[derive(Serialize, ToSchema)]
#[schema(example = json!({"status": "error", "message": "the reason for the error"}))]
pub struct Response {
    /// Status indicator (always `"error"` for error responses)
    #[schema(example = "error")]
    pub status: String,

    /// Human-readable error message safe to display to end users
    #[schema(example = "the reason for the error")]
    pub message: String,
}
