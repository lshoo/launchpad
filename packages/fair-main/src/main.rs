
use clap::{Arg, Command};

fn main() {
    let command = Command::new("Fair launchpad platform")
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

    let _matches = command.get_matches();
}