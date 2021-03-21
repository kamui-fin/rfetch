use clap::{App, Arg};
use colored::*;

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
    let colors = stats::color_scheme();
    let distro = stats::distro();
    let machine_info = stats::machine_info();
    let mem = stats::mem_info();
    let sys_info = stats::sysinfo();

    let mut output = String::from("");

    if let Some(user_info) = user_info {
        let user_host = format!(
            "{}{}{}",
            user_info.name.bold(),
            "@".magenta(),
            machine_info.nodename.bold()
        );
        output += &format!("{}\n", user_host);
        output += &format!(
            "{}\n",
            "━"
                .repeat(user_info.name.len() + machine_info.nodename.len() + 1)
                .magenta()
        );
        output += &format!(
            "{}   ~> {}\n",
            "sh".blue(),
            user_info.shell.to_str().unwrap()
        );
    }

    if let Some(distro) = distro {
        let pkgs = stats::packages(distro.name.as_str());
        output += &format!("{}   ~> {}\n", "os".blue(), distro.name);
        if let Some(pkgs) = pkgs {
            output += &format!("{} ~> {}\n", "pkgs".blue(), pkgs);
        }
    }

    if let Some(sys_info) = sys_info {
        let fmt_uptime = humantime::format_duration(sys_info.uptime).to_string();
        output += &format!("{}   ~> {}\n", "up".blue(), fmt_uptime);
    }

    if let Some(mem) = mem {
        output += &format!("{}  ~> {} / {}\n", "mem".blue(), mem.used, mem.total);
    }

    output += &format!("{} ~> {}\n\n", "kern".blue(), machine_info.kernel,);

    for (indx, color) in colors.into_iter().enumerate() {
        if indx == 8 {
            break;
            // Uncomment this if you want background colors to be shown too
            // println!("")
        }
        output += &format!("{}  \x1B[0m", color.0);
    }

    output += "\n";

    println!("{}", output);
}
