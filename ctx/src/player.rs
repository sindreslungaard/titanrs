use tokio::sync::oneshot;
use anyhow::Result;

pub enum Command {
    TryAuthenticate {
        player_id: i32,
        access_token: String,
        response: oneshot::Sender<bool>
    },
}