use axum::{extract::State, http::StatusCode, Extension, Json};
use std::sync::Arc;

use crate::{
    api::{
        request::assets::AssetCreateRequest,
        response::{assets::AssetResponse, TokenClaims},
    },
    state::ApplicationState,
};

pub async fn create(
    Extension(_claims): Extension<TokenClaims>,
    State(_state): State<Arc<ApplicationState>>,
    Json(_payload): Json<AssetCreateRequest>,
) -> Result<Json<AssetResponse>, StatusCode> {
    todo!()
}
