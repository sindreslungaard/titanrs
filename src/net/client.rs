use tokio::sync::mpsc;
use warp::ws::Message;

use crate::core::user::User;

pub struct Client {
    pub id: usize,
    pub channel: mpsc::UnboundedSender<Message>,
    pub user: Option<User>,
}

impl Client {
    pub fn new(id: usize, channel: mpsc::UnboundedSender<Message>) -> Client {
        Client {
            id,
            user: None,
            channel,
        }
    }

    pub fn receive(&self, msg: Message) {
        println!("client received message")
    }
}
