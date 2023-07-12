use sqlx::mysql::MySqlPoolOptions;
use tokio::sync::oneshot;
use std::collections::HashMap;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use room::{manager::{RoomManager, Command::LoadRoom}, Room};

use config::Config;

mod config;

#[tokio::main]
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

    info!("try create room manager");

    let room_manager = RoomManager::new(pool.clone());

    let (tx, rx) = oneshot::channel();
    room_manager.send(LoadRoom { room_id: 1, response: tx }).await.unwrap();
    let room_data = rx.await.unwrap().unwrap();
    info!("room data: {:?}", room_data);


    

    /* let room = Room::new();
    room.start(); */

    info!("done");
}
