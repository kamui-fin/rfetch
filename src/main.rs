#[macro_use]
extern crate log;

extern crate clap;
extern crate pretty_env_logger;

use clap::{App, Arg};

mod stats;

fn main() {
    pretty_env_logger::init();

    let matches = App::new("rfetch")
        .version("0.1")
        .author("Kamui")
        .about("Customizable fetch program for Linux")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Provide custom config file path")
                .takes_value(true),
        )
        .get_matches();

    let conf_file = matches.value_of("config").unwrap_or("config.toml");
    info!("Config file path: {}", conf_file);
}
