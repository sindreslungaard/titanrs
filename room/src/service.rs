use sqlx::mysql::MySqlPool;
use std::collections::HashMap;
use tokio::sync::oneshot;
use tokio::sync::mpsc;

use crate::Room;
use crate::data::room_data::RoomData;

#[derive(Debug)]
pub enum Command {
    LoadRoom {
        room_id: i32,
        response: oneshot::Sender<Result<RoomData, i32>>
    },
}

pub struct Service {
    ctx: titan::ctx::Context,
    rooms: HashMap<i32, Room>,
}

impl Service {
    pub fn new(ctx: titan::ctx::Context) -> mpsc::Sender<Command> {
        let (tx, rx) = mpsc::channel(1);

        tokio::spawn(async move {
            Service {
                ctx,
                rooms: HashMap::new(),
            }.run(rx).await;
        });

        info!("Room manager started");

        tx
    }

    async fn run(&self, mut rx: mpsc::Receiver<Command>) {

        while let Some(msg) = rx.recv().await {
            match msg {
                Command::LoadRoom { room_id, response } => {
                    let _ = response.send(self.load_room(room_id).await);
                }
            };
        }

        //todo: clean up
    }

    async fn load_room(&self, room_id: i32) -> Result<RoomData, i32> {
        match Room::load(room_id, self.db_pool.clone()).await {
            Some(room) => Ok(room.data.clone()),
            None => Err(0)
        }
    }
}

