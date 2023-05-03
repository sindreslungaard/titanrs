use log;
use serde::{Deserialize, Serialize};
use std::fs;
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub log_level: String,
    pub mysql: Mysql,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mysql {
    pub db_host: String,
    pub db_port: i32,
    pub db_name: String,
    pub db_user: String,
    pub db_pass: String,
    pub max_connections: u32,
}

impl Config {
    pub fn load(file_path: String) -> Self {
        let file = fs::read_to_string(&file_path);

        let config = match file {
            Ok(content) => content,
            Err(_) => {
                info!("Error loading config, creating default");
                let default_config = Config::default().to_string();
                fs::write(file_path, &default_config).unwrap();
                default_config
            }
        };

        return Config::from_str(config);
    }

    fn to_string(self) -> String {
        return toml::to_string(&self).unwrap();
    }

    fn from_str(str: String) -> Self {
        let config: Config = match toml::from_str(&str) {
            Ok(d) => d,
            Err(error) => {
                panic!("Error parsing config file: {:?}", error)
            }
        };

        return config;
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            log_level: "info".to_string(),
            mysql: Mysql {
                db_host: "localhost".to_string(),
                db_port: 3306,
                db_name: "titan".to_string(),
                db_user: "root".to_string(),
                db_pass: "".to_string(),
                max_connections: 5,
            },
        }
    }
}
