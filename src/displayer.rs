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
        let sys_info = stats::sysinfo();
        let dt = stats::current_datetime();

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
                                "{} {} {}\n",
                                "pkgs".color(self.config.title_color.clone()),
                                self.config.delimiter,
                                pkgs
                            );
                        }
                    }
                }
                "uptime" => {
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
                "ip" => {
                    let ip_type = if self.config.ip.public {
                        stats::IpType::Public
                    } else {
                        stats::IpType::Private
                    };
                    let ip_info = stats::ip(ip_type);
                    if let Some(ip_info) = ip_info {
                        output += &format!(
                            "{}   {} {}\n",
                            "ip".color(self.config.title_color.clone()),
                            self.config.delimiter,
                            ip_info
                        );
                    }
                }
                "cpu" => {
                    let cpu_info = stats::cpu_info();
                    if let Some(cpu_info) = cpu_info {
                        output += &format!(
                            "{}  {} {}\n",
                            "cpu".color(self.config.title_color.clone()),
                            self.config.delimiter,
                            cpu_info[0].model_name
                        );
                    }
                }
                "disk_usage" => {
                    let disk_usage = stats::disk_usage("/");
                    if let Some(disk_usage) = disk_usage {
                        output += &format!(
                            "{} {} {} / {}\n",
                            "disk".color(self.config.title_color.clone()),
                            self.config.delimiter,
                            disk_usage.used,
                            disk_usage.total_size
                        );
                    }
                }
                "process_num" => {
                    if let Some(sys_info) = &sys_info {
                        output += &format!(
                            "{} {} {}\n",
                            "proc".color(self.config.title_color.clone()),
                            self.config.delimiter,
                            sys_info.process_num
                        );
                    }
                }
                "arch" => {
                    output += &format!(
                        "{} {} {}\n",
                        "arch".color(self.config.title_color.clone()),
                        self.config.delimiter,
                        machine_info.arch
                    );
                }
                "temp" => {
                    let temp = stats::get_temp();
                    if let Some(temp) = temp {
                        output += &format!(
                            "{} {} {}°C\n",
                            "temp".color(self.config.title_color.clone()),
                            self.config.delimiter,
                            temp.0 / 1000
                        );
                    }
                }
                "locale" => {
                    let locale = stats::locale();
                    if let Some(locale) = locale {
                        output += &format!(
                            "{}  {} {}\n",
                            "loc".color(self.config.title_color.clone()),
                            self.config.delimiter,
                            locale.locale
                        );
                    }
                }
                "device_name" => {
                    let dev_n = stats::device();
                    if let Some(dev_n) = dev_n {
                        output += &format!(
                            "{} {} {}\n",
                            "host".color(self.config.title_color.clone()),
                            self.config.delimiter,
                            dev_n.0
                        );
                    }
                }
                "time" => {
                    let dt = stats::current_datetime();
                    output += &format!(
                        "{} {} {}\n",
                        "time".color(self.config.title_color.clone()),
                        self.config.delimiter,
                        dt.format("%k:%M %P")
                    );
                }
                "date" => {
                    output += &format!(
                        "{} {} {}\n",
                        "date".color(self.config.title_color.clone()),
                        self.config.delimiter,
                        dt.format("%b %d %Y")
                    );
                }
                _ => {}
            }
        }

        if self.config.colors.enabled {
            output += "\n";
            output += self.colors().as_str();
        }

        println!("{}", output);
    }
}
