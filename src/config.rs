use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;

use xdg;

use toml;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {}

impl Config {
    pub fn new() -> Config {
        Config {}
    }
}

pub struct TskConfig {
    pub location: PathBuf,
    pub config: Config,
}

impl TskConfig {
    pub fn new() -> TskConfig {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("tsk").unwrap();
        let location = xdg_dirs.place_config_file("config.toml").unwrap();
        
        let mut tsk_config = TskConfig {
            location: location.clone(),
            config: Config::new(),
        };

        if location.exists() {
            tsk_config.read();
        } else {
            tsk_config.write();
        }

        tsk_config
    }

    pub fn read(&mut self) {
        let mut config_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(self.location.clone())
            .unwrap();
        let mut buf = String::new();
        config_file
            .read_to_string(&mut buf)
            .expect("Could not read config");
        self.config = toml::from_str(&buf).expect("Could not parse config")
    }

    pub fn write(&mut self) {
        let mut config_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(self.location.clone())
            .unwrap();
        let toml = toml::to_string_pretty(&self.config).expect("Config corrupt");
        write!(config_file, "{}", toml);
    }
}
