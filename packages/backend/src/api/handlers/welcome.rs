use axum::http::StatusCode;

pub async fn welcome() -> Result<String, StatusCode> {
    Ok("\nWelcome\n".to_string())
}
