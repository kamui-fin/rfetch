use crate::config::Config;
use crate::stats;
use colored::*;

pub struct Displayer {
    config: Config,
}

impl Displayer {
    pub fn new(config: Config) -> Self {
        Displayer { config }
    }

    pub fn colors(self) -> String {
        let mut clrs = String::from("");
        let colors = stats::color_scheme();
        for (indx, color) in colors.into_iter().enumerate() {
            if indx == 8 {
                if self.config.show_bg_colors {
                    clrs += "\n"
                } else {
                    break;
                }
            }
            clrs += &format!("{}  \x1B[0m", color.0);
        }

        clrs += "\n";
        clrs
    }

    pub fn fetch(self) {
        let user_info = stats::user_info();
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
                "{}   {} {}\n",
                "sh".blue(),
                self.config.delimiter,
                user_info.shell.to_str().unwrap()
            );
        }

        if let Some(distro) = distro {
            let pkgs = stats::packages(distro.name.as_str());
            output += &format!(
                "{}   {} {}\n",
                "os".blue(),
                self.config.delimiter,
                distro.name
            );
            if let Some(pkgs) = pkgs {
                output += &format!("{} ~> {}\n", "pkgs".blue(), pkgs);
            }
        }

        if let Some(sys_info) = sys_info {
            let fmt_uptime = humantime::format_duration(sys_info.uptime).to_string();
            output += &format!(
                "{}   {} {}\n",
                "up".blue(),
                self.config.delimiter,
                fmt_uptime
            );
        }

        if let Some(mem) = mem {
            output += &format!(
                "{}  {} {} / {}\n",
                "mem".blue(),
                self.config.delimiter,
                mem.used,
                mem.total
            );
        }

        output += &format!(
            "{} {} {}\n\n",
            "kern".blue(),
            self.config.delimiter,
            machine_info.kernel,
        );

        output += self.colors().as_str();

        println!("{}", output);
    }
}
