use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub modules: Vec<String>,
    pub delimiter: String,
    pub title_color: String,
    pub colors: ColorConfig,
    pub user_host: UserHostConfig,
}

#[derive(Deserialize, Debug)]
pub struct ColorConfig {
    pub show_bg_colors: bool,
}

#[derive(Deserialize, Debug)]
pub struct UserHostConfig {
    pub line: bool,
    pub line_symbol: String,
    pub line_color: String,
}

impl Config {
    pub fn new() -> Self {
        let text = fs::read_to_string("config.toml").expect("Could not find config.toml");
        toml::from_str(text.as_str()).expect("Unable to parse config file")
    }
}
