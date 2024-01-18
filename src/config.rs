use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub modules: Vec<String>,
    pub delimiter: String,
    pub title_color: String,
    pub colors: ColorConfig,
    pub user_host: UserHostConfig,
    pub ip: IpConfig,
}

#[derive(Deserialize, Debug)]
pub struct ColorConfig {
    pub enabled: bool,
    pub show_bg_colors: bool,
}

#[derive(Deserialize, Debug)]
pub struct UserHostConfig {
    pub line: bool,
    pub line_symbol: String,
    pub line_color: String,
}

#[derive(Deserialize, Debug)]
pub struct IpConfig {
    pub public: bool,
}

impl Config {
    pub fn new(path: &str) -> Self {
        fn read_config(path: &str) -> Option<Result<Config, toml::de::Error>> {
            fs::read_to_string(path)
                .ok()
                .map(|config_str| toml::from_str(config_str.as_str()))
        }

        match read_config(path) {
            Some(config_read_result) => config_read_result.expect("Unable to parse config file"),
            None => Config::default(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            modules: vec![
                "user_host",
                "shell",
                "distro",
                "packages",
                "uptime",
                "memory",
                "kernel",
                "battery",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            delimiter: String::from("~>"),
            title_color: String::from("blue"),
            colors: ColorConfig {
                enabled: true,
                show_bg_colors: false,
            },
            user_host: UserHostConfig {
                line: true,
                line_symbol: String::from("-"),
                line_color: String::from("magenta"),
            },
            ip: IpConfig { public: false },
        }
    }
}
