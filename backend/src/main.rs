use std::time::Duration;

use axum::{
    Router,
    error_handling::HandleErrorLayer,
    http::{
        Method, StatusCode,
        header::{
            ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
            ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE, ORIGIN,
        },
    },
    response::IntoResponse,
    routing::get,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    BoxError,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::todo::create_todos_router;

mod todo;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().expect("failed to initialize .env");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = dotenvy::var("ADDR").expect("environment variable 'ADDR' not specified");
    let listener = TcpListener::bind(&addr).await?;
    let router = {
        let health_router = create_health_router();
        let todo_router = create_todos_router().await;
        let layers = ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|error: BoxError| async move {
                if error.is::<tower::timeout::error::Elapsed>() {
                    Ok(StatusCode::REQUEST_TIMEOUT)
                } else {
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("unhandled internal error: {error}"),
                    ))
                }
            }))
            .timeout(Duration::from_secs(10))
            .layer(TraceLayer::new_for_http())
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_headers([
                        ORIGIN,
                        CONTENT_TYPE,
                        AUTHORIZATION,
                        ACCESS_CONTROL_ALLOW_ORIGIN,
                        ACCESS_CONTROL_ALLOW_HEADERS,
                        ACCESS_CONTROL_ALLOW_METHODS,
                    ])
                    .allow_methods([
                        Method::GET,
                        Method::POST,
                        Method::PUT,
                        Method::DELETE,
                        Method::OPTIONS,
                    ]),
            )
            .into_inner();

        let router = Router::new()
            .merge(health_router)
            .merge(todo_router)
            .layer(layers);

        Router::new().nest("/api/v1", router)
    };

    tracing::info!("Running server on {addr:?}");
    axum::serve(listener, router).await?;

    Ok(())
}

pub fn create_health_router() -> Router {
    Router::new().route("/ping", get(ping))
}

pub async fn ping() -> impl IntoResponse {
    StatusCode::OK
}
