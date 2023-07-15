use ctx::Context;
use sqlx::mysql::MySqlPoolOptions;
use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::{mpsc, oneshot};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use room::{service::Service, Room};

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
    let db_pool = MySqlPoolOptions::new()
        .max_connections(config.mysql.max_connections)
        .connect(&connection_string)
        .await
        .unwrap();

    let (room_tx, room_rx) = mpsc::channel(1);

    let ctx = Context {
        db_pool,
        room_service: room_tx,
    };

    room::service::start(ctx.clone(), room_rx);

    let (tx, rx) = oneshot::channel();
    let _ = ctx
        .room_service
        .send(ctx::room::Command::LoadRoom {
            room_id: 1,
            response: tx,
        })
        .await;
    let room_data = rx.await.unwrap().unwrap();

    debug!("room id: {}", room_data.room_id);

    server::start(SocketAddr::from(([127, 0, 0, 1], 3030))).await;

    /* let room = Room::new();
    room.start(); */

    info!("done");
}
