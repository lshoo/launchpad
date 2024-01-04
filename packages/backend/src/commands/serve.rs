use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use clap::{value_parser, Arg, ArgMatches, Command};

use crate::{api, setting::Settings, state::ApplicationState};

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

pub fn handle(matches: &ArgMatches, setting: &Settings) -> anyhow::Result<()> {
    if let Some(matches) = matches.subcommand_matches(SERVE_NAME) {
        let port = *matches.get_one(PORT_NAME).unwrap_or(&PORT_DEFAULT_VALUE);

        start_server(port, setting)?;

        tracing::info!("Http Server started on port {}", port);
    }

    Ok(())
}

fn start_server(port: u16, settings: &Settings) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let state = Arc::new(ApplicationState::new(settings)?);

            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

            let routes = api::configure(state);

            axum::Server::bind(&addr)
                .serve(routes.into_make_service())
                .await?;

            Ok::<(), anyhow::Error>(())
        })?;

    std::process::exit(0)
}
