pub mod handlers;
pub mod request;
pub mod response;
pub mod v1;

use std::sync::Arc;

use axum::Router;

use crate::state::ApplicationState;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new().nest("/v1", v1::configure(state))
}
