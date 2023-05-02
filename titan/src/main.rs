use std::collections::HashMap;

use actix::{Actor, System};
use room::Room;

use config::Config;

mod config;

#[actix::main]
async fn main() {
    println!("Starting...");

    let config = Config::load("./config.toml".to_string());

    println!("config: {:?}", config);

    let room = Room::new();
    room.start();

    println!("done");
}
