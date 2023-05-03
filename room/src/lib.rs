#[macro_use]
extern crate sqlx;

use std::error::Error;

use actix::{Actor, Context, System};
use data::room_data::RoomData;
use sqlx::MySqlPool;

pub mod data;
pub mod manager;
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

        Some(Room { db_pool, data })
    }
}

impl Actor for Room {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Room process {} started", "-1");
        System::current().stop(); // <- stop system
    }
}
