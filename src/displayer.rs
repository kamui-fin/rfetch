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

    fn colors(&self) -> String {
        let mut clrs = String::from("");
        let colors = stats::color_scheme();
        for (indx, color) in colors.into_iter().enumerate() {
            if indx == 8 {
                if self.config.colors.show_bg_colors {
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

    pub fn fetch(&self) {
        let user_info = stats::user_info();
        let distro = stats::distro();
        let machine_info = stats::machine_info();

        let mut output = String::from("");

        for module in &self.config.modules {
            match module.as_str() {
                "user_host" => {
                    if let Some(user_info) = &user_info {
                        let user_host = format!(
                            "{}{}{}",
                            user_info.name.bold(),
                            "@".magenta(),
                            machine_info.nodename.bold()
                        );
                        output += &format!("{}\n", user_host);
                        if self.config.user_host.line {
                            output += &format!(
                                "{}\n",
                                self.config
                                    .user_host
                                    .line_symbol
                                    .repeat(user_info.name.len() + machine_info.nodename.len() + 1)
                                    .color(self.config.user_host.line_color.clone())
                            );
                        }
                    }
                }
                "shell" => {
                    if let Some(user_info) = &user_info {
                        output += &format!(
                            "{}   {} {}\n",
                            "sh".color(self.config.title_color.clone()),
                            self.config.delimiter,
                            user_info.shell.to_str().unwrap()
                        );
                    }
                }
                "distro" => {
                    if let Some(distro) = &distro {
                        output += &format!(
                            "{}   {} {}\n",
                            "os".color(self.config.title_color.clone()),
                            self.config.delimiter,
                            distro.name
                        );
                    }
                }
                "packages" => {
                    if let Some(distro) = &distro {
                        let pkgs = stats::packages(distro.name.as_str());
                        if let Some(pkgs) = pkgs {
                            output += &format!(
                                "{} ~> {}\n",
                                "pkgs".color(self.config.title_color.clone()),
                                pkgs
                            );
                        }
                    }
                }
                "uptime" => {
                    let sys_info = stats::sysinfo();
                    if let Some(sys_info) = &sys_info {
                        let fmt_uptime = humantime::format_duration(sys_info.uptime).to_string();
                        output += &format!(
                            "{}   {} {}\n",
                            "up".color(self.config.title_color.clone()),
                            self.config.delimiter,
                            fmt_uptime
                        );
                    }
                }
                "memory" => {
                    let mem = stats::mem_info();
                    if let Some(mem) = &mem {
                        output += &format!(
                            "{}  {} {} / {}\n",
                            "mem".color(self.config.title_color.clone()),
                            self.config.delimiter,
                            mem.used,
                            mem.total
                        );
                    }
                }
                "kernel" => {
                    output += &format!(
                        "{} {} {}\n",
                        "kern".color(self.config.title_color.clone()),
                        self.config.delimiter,
                        machine_info.kernel,
                    );
                }
                "colors" => {
                    output += "\n";
                    output += self.colors().as_str();
                }
                _ => {}
            }
        }

        println!("{}", output);
    }
}
