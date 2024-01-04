use clap::{Arg, Command};
use fair_main::commands;

fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let mut command = Command::new("Fair launchpad platform")
        .version("1.0")
        .author("lshoo <lshoo36@gmail.com>")
        .about("A fair launchpad platform for Digital Assets")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Configuration file location")
                .default_value("config.yaml"),
        );

    command = commands::configure(command);

    let matches = command.get_matches();

    commands::handle(&matches)?;

    Ok(())
}
