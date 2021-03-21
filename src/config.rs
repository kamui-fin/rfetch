use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub modules: Vec<String>,
    pub show_bg_colors: bool,
    pub delimiter: String,
}

impl Config {
    pub fn new() -> Self {
        let text = fs::read_to_string("config.toml").expect("Could not find config.toml");
        toml::from_str(text.as_str()).expect("Unable to parse config file")
    }
}
