use domain::user::UserInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SignupResponse {
    pub status: String,
    pub data: UserInfo,
}
