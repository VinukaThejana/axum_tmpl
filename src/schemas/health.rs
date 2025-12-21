use serde::Serialize;
use utoipa::ToSchema;

/// Health check response
#[derive(Serialize, ToSchema)]
#[schema(example = json!({"status": "healthy"}))]
pub struct Response {
    /// Current status of the service (e.g. `"healthy"`, `"degraded"`)
    #[schema(example = "healthy")]
    pub status: &'static str,
}

impl Response {
    pub fn new(status: &'static str) -> Self {
        Response { status }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Schema documentation tips:
// ──────────────────────────────────────────────────────────────────────────────
//
// 1. Use doc comments (`///`) on the struct and each field — these become
//    the description in the Swagger UI.
//
// 2. Use `#[schema(example = ...)]` on fields for inline examples, or
//    `#[schema(example = json!(...))]` on the struct for a full object example.
//
// 3. For request body schemas add validation + schema hints:
//
//    #[derive(Deserialize, ToSchema, Validate)]
//    pub struct CreateRequest {
//        /// User's display name
//        #[schema(example = "Jane Doe", min_length = 1, max_length = 100)]
//        #[validate(length(min = 1, max = 100))]
//        pub name: String,
//
//        /// User's email address
//        #[schema(example = "jane@example.com", format = "email")]
//        #[validate(email)]
//        pub email: String,
//    }
//
// 4. For enums, derive `ToSchema` and each variant becomes a possible value:
//
//    #[derive(Serialize, ToSchema)]
//    pub enum Role {
//        /// Standard user
//        User,
//        /// Administrator with elevated privileges
//        Admin,
//    }
