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
    let distro = stats::distro();
    let colors = stats::color_scheme();
    let machine_info = stats::machine_info();
    let mem = stats::mem_info();
    let sys_info = stats::sysinfo();

    if let Some(user_info) = user_info {
        let user_host = format!("{}@{}", user_info.name, machine_info.nodename);
        println!("{}", user_host);
        println!("{}", "-".repeat(user_host.len()));
        println!("Shell: {}", user_info.shell.to_str().unwrap());
    }

    if let Some(distro) = distro {
        let pkgs = stats::packages(distro.name.as_str());
        println!("OS: {}", distro.name);
        if let Some(pkgs) = pkgs {
            println!("Packages: {}", pkgs);
        }
    }

    if let Some(sys_info) = sys_info {
        let fmt_uptime = humantime::format_duration(sys_info.uptime).to_string();
        println!("Uptime: {}", fmt_uptime);
    }

    if let Some(mem) = mem {
        println!("Memory: {} / {}", mem.used, mem.total);
    }

    println!("Kernel: {}", machine_info.kernel);

    // // TODO: Add ascii art logos
    // // println!(r"    .--.");
    // // println!(r"   |o_o |");
    // // println!(r"   |:_/ |");
    // // println!(r"  //   \ \");
    // // println!(r" (|     | )");
    // // println!(r"/'\_   _/`\");
    // // println!(r"\___)=(___/");

    println!("");

    for (indx, color) in colors.into_iter().enumerate() {
        if indx == 8 {
            break;
            // Uncomment this if you want background colors to be shown too
            // println!("")
        }
        print!("{}  \x1B[0m", color.0,);
    }

    println!("");
}
