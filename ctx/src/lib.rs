use sqlx::MySqlPool;
use tokio::sync::mpsc;

pub mod room;
pub mod player;

#[derive(Clone)]
pub struct Context {
    pub db_pool: MySqlPool,
    pub room_service: mpsc::Sender<room::Command>
}