use clap::{Arg, Command};
use launchpad::{
    commands,
    setting::{Settings, CONFIG_FILE_NAME, CONFIG_NAME, ENV_PREFIX},
};
use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::{layer::SubscriberExt, Registry};

fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let mut command = Command::new("Fair launchpad platform")
        .version("1.0")
        .author("lshoo <lshoo36@gmail.com>")
        .about("A fair launchpad platform for Digital Assets")
        .arg(
            Arg::new(CONFIG_NAME)
                .short('c')
                .long(CONFIG_NAME)
                .help("Configuration file location")
                .default_value(CONFIG_FILE_NAME),
        );

    command = commands::configure(command);

    let matches = command.get_matches();

    let config_location = matches
        .get_one::<String>(CONFIG_NAME)
        .map(|s| s.as_str())
        .unwrap_or("");

    let settings = Settings::new(config_location, ENV_PREFIX)?;

    let subscriber = Registry::default()
        .with(LevelFilter::from_level(Level::DEBUG))
        .with(tracing_subscriber::fmt::Layer::default().with_writer(std::io::stdout));

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set log subscriber");

    tracing::info!(
        "db url: {:?}",
        settings
            .database
            .url
            .unwrap_or("Missing database url".to_string())
    );

    tracing::info!(
        "logging level: {:?}",
        settings.logging.log_level.unwrap_or("info".to_string())
    );

    // commands::handle(&matches, &settings)?;

    Ok(())
}
