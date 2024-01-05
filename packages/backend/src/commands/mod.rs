mod createadmin;
mod migrate;
mod serve;
mod welcome;

use clap::{ArgMatches, Command};

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
