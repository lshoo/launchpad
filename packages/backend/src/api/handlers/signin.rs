use axum::{extract::State, http::StatusCode, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::sync::Arc;

use crate::{
    api::{
        request::LoginRequest,
        response::{LoginResponse, TokenClaims},
    },
    state::ApplicationState,
    validate_user_password,
};

pub async fn login(
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    validate_user_password(&state.db_conn.load(), &payload.username, &payload.password)
        .await
        .expect("Verify password failed");

    let secret = &state.settings.load().token_secret;
    let expired_secs = state.settings.load().token_expiration;

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::seconds(expired_secs)).timestamp() as usize;

    let claims = TokenClaims {
        sub: payload.username,
        exp,
        iat,
    };

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
