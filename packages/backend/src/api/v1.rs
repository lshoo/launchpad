use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::state::ApplicationState;

use super::handlers;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route(
            "/welcome",
            get(handlers::welcome::welcome).with_state(state.clone()),
        )
        .route(
            "/login",
            post(handlers::signin::login).with_state(state.clone()),
        )
        .route(
            "/signup",
            post(handlers::signup::register).with_state(state.clone()),
        )
}
