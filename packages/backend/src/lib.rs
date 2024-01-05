use argon2::Argon2;
use password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use sea_orm::{Database, DatabaseConnection};
use setting::Settings;

pub mod api;

pub mod commands;

pub mod setting;
pub mod state;

pub fn to_static_str(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

pub async fn get_conn(settings: &Settings) -> anyhow::Result<DatabaseConnection> {
    let url = settings.db_url().expect("Missing database url");
    Ok(Database::connect(&url)
        .await
        .expect("Database connection failed"))
}

pub fn encrypt_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    if let Ok(hash) = argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash.to_string())
    } else {
        Err(anyhow::anyhow!("Failed to hash password"))
    }
}

pub fn validate_password(password: &str, hashed: &str) -> anyhow::Result<()> {
    let argon2 = Argon2::default();
    let hashed_password = PasswordHash::new(hashed).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    argon2
        .verify_password(password.as_bytes(), &hashed_password)
        .map_err(|e| anyhow::anyhow!(format!("Failed to verify password: {}", e.to_string())))
}

pub async fn validate_user_password(
    db: &DatabaseConnection,
    username: &str,
    password: &str,
) -> anyhow::Result<bool> {
    let user = domain::user::find_by_username(db, username).await?;

    if let Some(user) = user {
        validate_password(password, &user.password)?;

        return Ok(true);
    }

    Ok(false)
}
