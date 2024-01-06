use sea_orm::DatabaseConnection;

pub mod signin;
pub mod signup;
pub mod welcome;

/// Check the username is valid, and does not exists
pub async fn is_valid_username(db: &DatabaseConnection, username: &str) -> anyhow::Result<()> {
    let user = domain::user::find_by_username(db, username).await?;

    if user.is_none() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("User already exists"))
    }
}
