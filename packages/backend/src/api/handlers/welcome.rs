use std::sync::Arc;

use axum::{extract::State, http::StatusCode};

use crate::state::ApplicationState;

pub async fn welcome(State(state): State<Arc<ApplicationState>>) -> Result<String, StatusCode> {
    Ok(format!(
        "\nWelcome to Launchpad platform using config: {}\n\n",
        state
            .settings
            .load()
            .config
            .location
            .clone()
            .unwrap_or("".to_string())
    ))
}
