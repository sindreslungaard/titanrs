use sqlx::mysql::MySqlPool;
use std::collections::HashMap;

use actix::{Actor, Addr, Context};

use crate::Room;

pub struct RoomManager {
    db_pool: MySqlPool,
    rooms: HashMap<i32, Addr<Room>>,
}

impl RoomManager {
    pub fn new(db_pool: MySqlPool) -> RoomManager {
        RoomManager {
            db_pool,
            rooms: HashMap::new(),
        }
    }
}

impl Actor for RoomManager {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Room manager started");
    }
}
