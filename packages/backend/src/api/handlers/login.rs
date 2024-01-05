use axum::{extract::State, http::StatusCode, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::sync::Arc;

use crate::{
    api::{
        request::LoginRequest,
        response::{LoginResponse, TokenClaims},
    },
    state::ApplicationState,
};

pub async fn login(
    State(_state): State<Arc<ApplicationState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(120)).timestamp() as usize;

    let claims = TokenClaims {
        sub: payload.username,
        exp,
        iat,
    };

    let secret = "whoami";

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();

    let response = LoginResponse {
        status: "success".to_string(),
        token,
    };

    Ok(Json(response))
}
