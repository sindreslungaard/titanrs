use std::collections::HashMap;

use actix::{Actor, System};
use room::Room;

#[actix::main]
async fn main() {
    println!("hello world");

    let room = Room::new();
    room.start();

    println!("done");
}
