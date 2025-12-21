use crate::schemas::health;
use axum::{
    Json,
    http::{StatusCode, header},
    response::IntoResponse,
};

/// Health check
///
/// Returns the current health status of the service.
/// Use this endpoint for load balancer health checks,
/// container orchestration readiness probes (Kubernetes, ECS, etc.),
/// and uptime monitoring systems.
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    operation_id = "check_health",
    responses(
        (status = 200, description = "Service is healthy", body = health::Response),
        (status = 500, description = "Service is unhealthy", body = health::Response)
    )
)]
pub async fn health() -> impl IntoResponse {
    // NOTE: Add health check logic for all third party services here

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        Json(health::Response::new("healthy")),
    )
}

// ──────────────────────────────────────────────────────────────────────────────
// Example: A protected route with path params, query params, and security
// ──────────────────────────────────────────────────────────────────────────────
//
// use crate::schemas::user;
// use axum::extract::{Path, Query};
// use serde::Deserialize;
// use utoipa::IntoParams;
//
// #[derive(Deserialize, IntoParams)]
// pub struct PaginationParams {
//     /// Page number (1-indexed)
//     #[param(example = 1, minimum = 1)]
//     pub page: u32,
//     /// Number of items per page
//     #[param(example = 20, maximum = 100)]
//     pub per_page: u32,
// }
//
// /// Get user by ID
// ///
// /// Retrieves the full profile of a user by their unique identifier.
// /// Requires a valid bearer token.
// #[utoipa::path(
//     get,
//     path = "/users/{id}",
//     tag = "Users",
//     operation_id = "get_user_by_id",
//     params(
//         ("id" = i64, Path, description = "Unique user identifier"),
//         PaginationParams
//     ),
//     responses(
//         (status = 200, description = "User found", body = user::Response),
//         (status = 401, description = "Unauthorized", body = schemas::error::Response),
//         (status = 404, description = "User not found", body = schemas::error::Response),
//         (status = 500, description = "Internal server error", body = schemas::error::Response),
//     ),
//     security(("bearer_auth" = []))
// )]
// pub async fn get_user(
//     Path(id): Path<i64>,
//     Query(pagination): Query<PaginationParams>,
// ) -> impl IntoResponse {
//     todo!()
// }
//
// ──────────────────────────────────────────────────────────────────────────────
// Example: A POST route with a JSON request body
// ──────────────────────────────────────────────────────────────────────────────
//
// use crate::schemas::user;
//
// /// Create a new user
// ///
// /// Registers a new user account with the provided details.
// /// Returns the created user on success.
// #[utoipa::path(
//     post,
//     path = "/users",
//     tag = "Users",
//     operation_id = "create_user",
//     request_body(
//         content = user::CreateRequest,
//         description = "User registration payload",
//         content_type = "application/json"
//     ),
//     responses(
//         (status = 201, description = "User created successfully", body = user::Response),
//         (status = 400, description = "Validation error", body = schemas::error::Response),
//         (status = 409, description = "User already exists", body = schemas::error::Response),
//         (status = 500, description = "Internal server error", body = schemas::error::Response),
//     )
// )]
// pub async fn create_user(
//     Json(body): Json<user::CreateRequest>,
// ) -> impl IntoResponse {
//     todo!()
// }
