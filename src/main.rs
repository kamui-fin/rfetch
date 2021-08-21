use clap::{App, Arg};
use std::path::Path;

mod config;
mod displayer;
mod stats;

fn main() {
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

    let home = stats::get_env("HOME").expect("HOME variable has not been set");
    let def_conf_path = home + "/.config/rfetch/config.toml";
    let conf_path = if let Some(config) = matches.value_of("config") {
        config
    } else if Path::new(&def_conf_path).exists() {
        def_conf_path.as_str()
    } else {
        "config.toml"
    };
    let conf = config::Config::new(conf_path);

    let displayer = displayer::Displayer::new(conf);
    displayer.fetch();
}
