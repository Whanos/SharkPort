use serde_derive::{Deserialize};
use std::fs;
use toml;

#[derive(Deserialize)]
pub(crate) struct Data {
    pub(crate) skinport: Skinport,
    pub(crate) discord: Discord,
}

#[derive(Deserialize)]
pub(crate) struct Skinport {
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) websocket_url: String,
}

#[derive(Deserialize)]
pub(crate) struct Discord {
    pub(crate) role_id: String,
    pub(crate) skinport_url: String,
}

pub(crate) fn read_config() -> Data {
    let config_file = fs::read_to_string("config.toml")
        .expect("Unable to read config file");
    let config_data: Data = toml::from_str(&config_file)
        .expect("Unable to parse config file");

    config_data
}