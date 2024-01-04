use clap::{ArgMatches, Command};

pub fn configure() -> Command {
    Command::new("welcome").about("Welcome to fair launchpad")
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    if let Some(_matches) = matches.subcommand_matches("welcome") {
        println!("Welcome to fair launchpad!");
    }

    Ok(())
}
