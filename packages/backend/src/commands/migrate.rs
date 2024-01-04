use clap::{ArgMatches, Command};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

use crate::setting::Settings;

pub fn configure() -> Command {
    Command::new("migrate").about("Migrating database")
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    if let Some(_matches) = matches.subcommand_matches("migrate") {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let db_url = settings.database.url.clone().expect("Missing database url");
                let conn = Database::connect(db_url)
                    .await
                    .expect("Database connection failed");

                Migrator::up(&conn, None).await.unwrap()
            });
    }

    Ok(())
}
