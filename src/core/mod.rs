pub mod room;
pub mod user;

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use tokio::sync::RwLock;

use crate::net::client::Client;

use self::room::Room;
use self::user::User;

pub type Users = Arc<RwLock<HashMap<usize, User>>>;
pub type Rooms = Arc<RwLock<HashMap<usize, Room>>>;
pub type Clients = Arc<RwLock<HashMap<usize, Client>>>;

pub struct State {
    pub users: Users,
    pub rooms: Rooms,
    pub clients: Clients,
}

impl State {
    pub fn new() -> Arc<State> {
        Arc::new(State {
            users: Users::default(),
            rooms: Rooms::default(),
            clients: Clients::default(),
        })
    }
}
