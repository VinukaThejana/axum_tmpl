use crate::{handler, schemas};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{Http, HttpAuthScheme, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::health
        // handler::get_user,
        // handler::create_user,
    ),
    components(
        schemas(
            schemas::error::Response,
            schemas::health::Response
            // schemas::user::Response,
            // schemas::user::CreateRequest,
        )
    ),
    servers(
        (url = "/api", description = "API Gateway")
    ),
    tags(
        (name = "Health", description = "Service health and readiness endpoints"),
        // (name = "Users", description = "User management endpoints"),
        // (name = "Auth", description = "Authentication and authorization"),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearer_auth", // This name must match the one used in #[utoipa::path(security(...))]
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
    }
}
