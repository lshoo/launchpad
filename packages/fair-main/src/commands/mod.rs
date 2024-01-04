mod serve;
mod welcome;

use clap::{ArgMatches, Command};

pub fn configure(command: Command) -> Command {
    command
        .subcommand(welcome::configure())
        .subcommand(serve::configure())
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    welcome::handle(matches)?;
    serve::handle(matches)?;

    Ok(())
}

pub fn to_static_str(num: &str) -> &'static str {
    Box::leak(num.to_string().into_boxed_str())
}
