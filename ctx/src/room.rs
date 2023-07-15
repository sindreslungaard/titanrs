use tokio::sync::oneshot;
use anyhow::Result;

pub struct LoadRoomResponse {
    pub room_id: i32,
}

pub enum Command {
    LoadRoom {
        room_id: i32,
        response: oneshot::Sender<Result<LoadRoomResponse>>
    },
}