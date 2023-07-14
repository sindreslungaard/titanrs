use sqlx::MySqlPool;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct Context {
    db_pool: MySqlPool,
    room_service: mpsc::Sender<room::service::Command>
}