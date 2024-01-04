mod serve;
mod welcome;

use clap::{ArgMatches, Command};

use crate::setting::Settings;

pub fn configure(command: Command) -> Command {
    command
        .subcommand(welcome::configure())
        .subcommand(serve::configure())
}

pub fn handle(matches: &ArgMatches, setting: &Settings) -> anyhow::Result<()> {
    welcome::handle(matches)?;
    serve::handle(matches, setting)?;

    Ok(())
}

pub fn to_static_str(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}
