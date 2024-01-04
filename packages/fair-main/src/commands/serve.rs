use clap::{value_parser, Arg, ArgMatches, Command};

static SERVE_NAME: &str = "serve";
static PORT_NAME: &str = "port";
static PORT_VALUE_NAME: &str = "PORT";
static PORT_DEFAULT_VALUE: u16 = 8080;

pub fn configure() -> Command {
    Command::new(SERVE_NAME)
        .about("Start the fairpad server")
        .arg(
            Arg::new(PORT_NAME)
                .short('p')
                .long(PORT_NAME)
                .value_name(PORT_VALUE_NAME)
                .help("TCP port is listen on")
                .default_value("8080")
                .value_parser(value_parser!(u16)),
        )
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    if let Some(matches) = matches.subcommand_matches(SERVE_NAME) {
        let port = matches.get_one(PORT_NAME).unwrap_or(&PORT_DEFAULT_VALUE);

        println!("TBD: Server started on port {}", port);
    }

    Ok(())
}
