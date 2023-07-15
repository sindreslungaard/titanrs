use sqlx::mysql::MySqlPool;
use std::collections::HashMap;
use tokio::sync::oneshot;
use tokio::sync::mpsc;
use ctx::room::*;
use anyhow::anyhow;

use crate::Room;
use crate::data::room_data::RoomData;

pub struct Service {
    ctx: ctx::Context,
    rooms: HashMap<i32, Room>,
}

impl Service {
    pub fn new(ctx: ctx::Context, rx: mpsc::Receiver<Command>) {
        tokio::spawn(async move {
            Service {
                ctx,
                rooms: HashMap::new(),
            }.run(rx).await;
        });

        info!("Room service started");
    }

    async fn run(&self, mut rx: mpsc::Receiver<ctx::room::Command>) {

        while let Some(msg) = rx.recv().await {
            match msg {
                Command::LoadRoom { room_id, response } => {
                    match self.load_room(room_id).await {
                        Ok(room) => {
                            let _ = response.send(Ok(LoadRoomResponse {
                                room_id: room.id
                            }));
                        }
                        Err(_) => {
                            let _ = response.send(Err(anyhow!("Test")));
                        }
                    }
                }
            };
        }

        //todo: clean up
    }

    async fn load_room(&self, room_id: i32) -> Result<RoomData, i32> {
        match Room::load(room_id, self.ctx.db_pool.clone()).await {
            Some(room) => Ok(room.data.clone()),
            None => Err(0)
        }
    }
}

