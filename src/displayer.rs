use crate::stats::{self, Distro};
use crate::{
    config::Config,
    stats::{MachineInfo, UserInfo},
};
use colored::*;
use stats::SysInfo;

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

    fn show_user_host(
        &self,
        user_info: &Option<UserInfo>,
        machine_info: &MachineInfo,
    ) -> Option<String> {
        if let Some(info) = user_info {
            let user_host = format!(
                "{}{}{}",
                info.name.bold(),
                "@".magenta(),
                machine_info.nodename.bold()
            );
            let mut output = format!("{}\n", user_host);
            if self.config.user_host.line {
                output.push_str(&format!(
                    "{}\n",
                    self.config
                        .user_host
                        .line_symbol
                        .repeat(info.name.len() + machine_info.nodename.len() + 1)
                        .color(self.config.user_host.line_color.clone())
                ));
            }
            return Some(output);
        }
        None
    }

    fn show_shell(&self, user_info: &Option<UserInfo>) -> Option<String> {
        if let Some(info) = user_info {
            let output = &format!(
                "{}   {} {}\n",
                "sh".color(self.config.title_color.clone()),
                self.config.delimiter,
                info.shell.to_str().unwrap()
            );
            return Some(String::from(output));
        }
        None
    }

    fn show_distro(&self, distro: &Option<Distro>) -> Option<String> {
        if let Some(distro) = &distro {
            let output = &format!(
                "{}   {} {}\n",
                "os".color(self.config.title_color.clone()),
                self.config.delimiter,
                distro.name
            );
            return Some(String::from(output));
        }
        None
    }

    fn show_packages(&self, distro: &Option<Distro>) -> Option<String> {
        if let Some(distro) = &distro {
            let pkgs = stats::packages(distro.name.as_str());
            if let Some(pkgs) = pkgs {
                let output = &format!(
                    "{} {} {}\n",
                    "pkgs".color(self.config.title_color.clone()),
                    self.config.delimiter,
                    pkgs
                );
                return Some(String::from(output));
            }
        }
        None
    }

    fn show_uptime(&self, sys_info: &Option<SysInfo>) -> Option<String> {
        if let Some(sys_info) = &sys_info {
            let fmt_uptime = humantime::format_duration(sys_info.uptime).to_string();
            let output = &format!(
                "{}   {} {}\n",
                "up".color(self.config.title_color.clone()),
                self.config.delimiter,
                fmt_uptime
            );
            return Some(String::from(output));
        }
        None
    }
    fn show_mem(&self) -> Option<String> {
        let mem = stats::mem_info();
        if let Some(mem) = &mem {
            let output = &format!(
                "{}  {} {} / {}\n",
                "mem".color(self.config.title_color.clone()),
                self.config.delimiter,
                mem.used,
                mem.total
            );
            return Some(String::from(output));
        }
        None
    }

    fn show_kern(&self, machine_info: &MachineInfo) -> String {
        let output = &format!(
            "{} {} {}\n",
            "kern".color(self.config.title_color.clone()),
            self.config.delimiter,
            machine_info.kernel,
        );
        String::from(output)
    }

    fn show_ip(&self) -> Option<String> {
        let ip_type = if self.config.ip.public {
            stats::IpType::Public
        } else {
            stats::IpType::Private
        };
        let ip_info = stats::ip(ip_type);
        let output = &format!(
            "{}   {} {}\n",
            "ip".color(self.config.title_color.clone()),
            self.config.delimiter,
            if let Some(ip_info) = ip_info {
                ip_info.to_string()
            } else {
                "not connected".to_string()
            }
        );
        Some(String::from(output))
    }

    fn show_cpu(&self) -> Option<String> {
        let cpu_info = stats::cpu_info();
        if let Some(cpu_info) = cpu_info {
            let output = &format!(
                "{}  {} {}\n",
                "cpu".color(self.config.title_color.clone()),
                self.config.delimiter,
                cpu_info[0].model_name
            );
            return Some(String::from(output));
        }
        None
    }

    fn show_disk(&self) -> Option<String> {
        let disk_usage = stats::disk_usage("/");
        if let Some(disk_usage) = disk_usage {
            let output = &format!(
                "{} {} {} / {}\n",
                "disk".color(self.config.title_color.clone()),
                self.config.delimiter,
                disk_usage.used,
                disk_usage.total_size
            );
            return Some(String::from(output));
        }
        None
    }

    fn show_process(&self, sys_info: &Option<SysInfo>) -> Option<String> {
        if let Some(sys_info) = &sys_info {
            let output = &format!(
                "{} {} {}\n",
                "proc".color(self.config.title_color.clone()),
                self.config.delimiter,
                sys_info.process_num
            );
            return Some(String::from(output));
        }
        None
    }

    fn show_arch(&self, machine_info: &MachineInfo) -> String {
        let output = &format!(
            "{} {} {}\n",
            "arch".color(self.config.title_color.clone()),
            self.config.delimiter,
            machine_info.arch
        );
        String::from(output)
    }

    fn show_temp(&self) -> Option<String> {
        let temp = stats::get_temp();
        if let Some(temp) = temp {
            let output = &format!(
                "{} {} {}°C\n",
                "temp".color(self.config.title_color.clone()),
                self.config.delimiter,
                temp.0 / 1000
            );
            return Some(String::from(output));
        }
        None
    }

    fn show_locale(&self) -> Option<String> {
        let locale = stats::locale();
        if let Some(locale) = locale {
            let output = &format!(
                "{}  {} {}\n",
                "loc".color(self.config.title_color.clone()),
                self.config.delimiter,
                locale.locale
            );
            return Some(String::from(output));
        }
        None
    }

    fn show_device(&self) -> Option<String> {
        let dev_n = stats::device();
        if let Some(dev_n) = dev_n {
            let output = &format!(
                "{} {} {}\n",
                "host".color(self.config.title_color.clone()),
                self.config.delimiter,
                dev_n.0
            );
            return Some(String::from(output));
        }
        None
    }

    fn show_time(&self) -> String {
        let dt = stats::current_datetime();
        let output = &format!(
            "{} {} {}\n",
            "time".color(self.config.title_color.clone()),
            self.config.delimiter,
            dt.format("%k:%M %P")
        );
        String::from(output)
    }

    fn show_date(&self) -> String {
        let dt = stats::current_datetime();
        let output = &format!(
            "{} {} {}\n",
            "date".color(self.config.title_color.clone()),
            self.config.delimiter,
            dt.format("%b %d %Y")
        );
        String::from(output)
    }

    pub fn fetch(&self) {
        let mut output = String::from("");
        let user_info = stats::user_info();
        let distro = stats::distro();
        let machine_info = stats::machine_info();
        let sys_info = stats::sysinfo();

        for module in &self.config.modules {
            match module.as_str() {
                "user_host" => {
                    output += &self.show_user_host(&user_info, &machine_info).unwrap();
                }
                "shell" => {
                    output += &self.show_shell(&user_info).unwrap();
                }
                "distro" => {
                    output += &self.show_distro(&distro).unwrap();
                }
                "packages" => {
                    output += &self.show_packages(&distro).unwrap();
                }
                "uptime" => {
                    output += &self.show_uptime(&sys_info).unwrap();
                }
                "memory" => {
                    output += &self.show_mem().unwrap();
                }
                "kernel" => {
                    output += &self.show_kern(&machine_info);
                }
                "ip" => {
                    output += &self.show_ip().unwrap_or("Not connected.".to_string());
                }
                "cpu" => {
                    output += &self.show_cpu().unwrap();
                }
                "disk_usage" => {
                    output += &self.show_disk().unwrap();
                }
                "process_num" => {
                    output += &self.show_process(&sys_info).unwrap();
                }
                "arch" => {
                    output += &self.show_arch(&machine_info);
                }
                "temp" => {
                    output += &self.show_temp().unwrap();
                }
                "locale" => {
                    output += &self.show_locale().unwrap();
                }
                "device_name" => {
                    output += &self.show_device().unwrap();
                }
                "time" => {
                    output += &self.show_time();
                }
                "date" => {
                    output += &self.show_date();
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
