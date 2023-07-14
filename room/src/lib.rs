#[macro_use]
extern crate sqlx;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::error::Error;

use data::room_data::RoomData;
use service::Service;
use sqlx::MySqlPool;

pub mod data;
pub mod service;

pub struct Room {
    db_pool: MySqlPool,
    pub data: RoomData,
}

impl Room {
    pub async fn load(id: i32, db_pool: MySqlPool) -> Option<Self> {
        let data = match RoomData::by_id(id, db_pool.clone()).await {
            Ok(d) => d,
            Err(_) => return None,
        };

        info!("Loaded room {}", data.id);

        Some(Room { db_pool, data })
    }
}