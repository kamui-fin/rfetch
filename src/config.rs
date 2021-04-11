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
        let text = fs::read_to_string(path).expect("Could not find config.toml");
        toml::from_str(text.as_str()).expect("Unable to parse config file")
    }
}
