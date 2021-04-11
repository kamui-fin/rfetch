use clap::{App, Arg};

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

    let conf_file = matches.value_of("config").unwrap_or("config.toml");
    let conf = config::Config::new(conf_file);

    let displayer = displayer::Displayer::new(conf);
    displayer.fetch();
}
