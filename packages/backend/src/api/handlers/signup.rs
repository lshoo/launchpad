use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, TryIntoModel};

use std::sync::Arc;

use crate::{
    api::{handlers::is_valid_username, request::SignupRequest, response::SignupResponse},
    encrypt_password,
    state::ApplicationState,
};

pub async fn register(
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<SignupResponse>, StatusCode> {
    let username = &payload.username;

    validate_confirm_password(&payload.password, &payload.confirm_password)
        .expect("Two password not same");

    let db_conn = state.db_conn.load();

    is_valid_username(&db_conn, username)
        .await
        .expect("Username invalid");

    let password = encrypt_password(&payload.password).expect("Encrypt password failed");

    let now = chrono::Utc::now().naive_utc();
    let user = domain::user::new_user_model(username.to_owned(), password, now, payload.wallet);

    let user = user
        .save(state.db_conn.load().as_ref())
        .await
        .expect("Save user failed");

    let user_info = user
        .try_into_model()
        .expect("User model convert failed")
        .user_info();

    Ok(Json(SignupResponse {
        status: "Success".to_string(),
        data: user_info,
    }))
}

fn validate_confirm_password(password: &str, confirm_password: &str) -> anyhow::Result<bool> {
    Ok(password == confirm_password)
}
