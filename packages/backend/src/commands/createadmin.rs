use clap::Arg;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::{
    commands::{encrypt_password, get_conn},
    setting::Settings,
};

pub(crate) fn configure() -> clap::Command {
    clap::Command::new("createadmin")
        .about("Create admin user")
        .arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .value_name("PASSWORD")
                .help("Admin password")
                .default_value("Password123"),
        )
}

pub(crate) fn handle(matches: &clap::ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    if let Some(matches) = matches.subcommand_matches("createadmin") {
        let password = matches.get_one::<String>("password").unwrap();

        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async move {
                let conn = get_conn(settings).await?;

                let admins = domain::user::Entity::find()
                    .filter(domain::user::Column::Username.eq("admin"))
                    .all(&conn)
                    .await?;

                if !admins.is_empty() {
                    tracing::info!("Admin user already exists");
                    return Ok::<(), anyhow::Error>(());
                }

                let password = encrypt_password(password)?;

                let admin = domain::user::ActiveModel {
                    username: Set("admin".to_owned()),
                    password: Set(password),
                    created_at: Set(chrono::Utc::now().naive_utc()),
                    ..Default::default()
                };

                if let Ok(_ad) = admin.save(&conn).await {
                    tracing::info!("Admin created");
                } else {
                    tracing::error!("Failed to create admin");
                }

                Ok(())
            })?;
    }

    Ok(())
}
