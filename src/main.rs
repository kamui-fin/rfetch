extern crate bytesize;
extern crate clap;

use clap::{App, Arg};

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

    let user_info = stats::user_info();
    let default_config = [
        user_info.home.clone(),
        String::from("/.config/rfetch/config.toml"),
    ]
    .join("");

    let _conf_file = matches
        .value_of("config")
        .unwrap_or(default_config.as_str());

    let distro = stats::distro();
    let colors = stats::color_scheme();
    let machine_info = stats::machine_info();
    let mem = stats::mem_info();
    let sys_info = stats::sysinfo();

    let fmt_uptime = humantime::format_duration(sys_info.uptime).to_string();

    // TODO: Add ascii art logos
    // println!(r"    .--.");
    // println!(r"   |o_o |");
    // println!(r"   |:_/ |");
    // println!(r"  //   \ \");
    // println!(r" (|     | )");
    // println!(r"/'\_   _/`\");
    // println!(r"\___)=(___/");

    let user_host = format!("{}@{}", user_info.name, machine_info.hostname);
    println!("{}", user_host);
    println!("{}", "-".repeat(user_host.len()));
    println!("OS:\t{}", distro.name);
    println!("Kernel:\t{}", machine_info.kernel);
    println!("Uptime:\t{}", fmt_uptime);
    println!("Shell:\t{}", user_info.shell);
    println!("Memory:\t{} / {}", mem.used, mem.total);
    println!("");

    for (indx, color) in colors.into_iter().enumerate() {
        if indx == 8 {
            println!("")
        }
        print!("{}  \x1B[0m", color.0,);
    }
    println!("");
}
