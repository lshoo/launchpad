mod createadmin;
mod migrate;
mod serve;
mod welcome;

use argon2::Argon2;
use clap::{ArgMatches, Command};
use password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use sea_orm::{Database, DatabaseConnection};

use crate::setting::Settings;

pub fn configure(command: Command) -> Command {
    command
        .subcommand(welcome::configure())
        .subcommand(serve::configure())
        .subcommand(migrate::configure())
        .subcommand(createadmin::configure())
}

pub fn handle(matches: &ArgMatches, setting: &Settings) -> anyhow::Result<()> {
    welcome::handle(matches)?;
    serve::handle(matches, setting)?;
    migrate::handle(matches, setting)?;
    createadmin::handle(matches, setting)?;

    Ok(())
}

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
