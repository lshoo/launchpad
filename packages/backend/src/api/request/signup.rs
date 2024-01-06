use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
    pub wallet: Option<String>,
}
