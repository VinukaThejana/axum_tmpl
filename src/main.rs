use ::log::info;
use axum::{Router, http::StatusCode, routing::get};
use axum_tmpl::{
    config::{ENV, log, state::AppState},
    doc::ApiDoc,
    handler,
    util::{governor_conf, governor_err, shutdown},
};
use envmode::EnvMode;
use std::{net::SocketAddr, time::Duration};
use tokio::{net::TcpListener, time};
use tower::ServiceBuilder;
use tower_governor::GovernorLayer;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log::setup();
    let state = AppState::new().await;

    let governor_conf = governor_conf();
    let governor_limiter = governor_conf.limiter().clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            info!(
                "[governor] limiting storage size: {}",
                governor_limiter.len()
            );
            governor_limiter.retain_recent();
        }
    });
    let governor_layer = GovernorLayer::new(governor_conf).error_handler(governor_err);

    let app = Router::new()
        .route("/health", get(handler::health))
        // TODO: Add your routes here
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(TimeoutLayer::with_status_code(
                    StatusCode::REQUEST_TIMEOUT,
                    Duration::from_secs(10 * 60),
                ))
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods(Any)
                        .allow_headers(Any),
                ),
        )
        .layer(governor_layer)
        .with_state(state);

    let router = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api", app);

    if EnvMode::is_dev(&ENV.environment) {
        info!("running on : http://localhost:{}", &ENV.port);
        info!("API doc : http://localhost:{}/swagger-ui", &ENV.port);
    }

    axum::serve(
        TcpListener::bind(&format!("0.0.0.0:{}", &ENV.port))
            .await
            .unwrap(),
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown())
    .await
    .unwrap();

    Ok(())
}
