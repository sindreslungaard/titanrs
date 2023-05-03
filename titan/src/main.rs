use sqlx::mysql::MySqlPoolOptions;
use std::collections::HashMap;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use actix::{Actor, System};
use room::Room;

use config::Config;

mod config;

#[actix::main]
async fn main() {
    let config = Config::load("./config.toml".to_string());

    std::env::set_var("RUST_LOG", config.log_level);
    pretty_env_logger::init();

    info!("Starting...");

    let connection_string = format!(
        "mysql://{}:{}@{}/{}",
        config.mysql.db_user, config.mysql.db_pass, config.mysql.db_host, config.mysql.db_name
    );

    info!("Creating mysql connection pool");
    let pool = MySqlPoolOptions::new()
        .max_connections(config.mysql.max_connections)
        .connect(&connection_string)
        .await
        .unwrap();

    let room = Room::new();
    room.start();

    info!("done");
}
