pub mod signin;
pub mod signup;

use serde::{Deserialize, Serialize};
pub use signin::*;
pub use signup::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
