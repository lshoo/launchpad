use axum::Router;

pub mod handlers;

pub mod v1;

pub fn configure() -> Router {
    Router::new().nest("/v1", v1::configure())
}
