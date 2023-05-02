use actix::{Actor, Context, System};

pub mod manager;

#[derive(Clone)]
pub struct RoomData {
    _id: i32,
    _owner: i32,
    _name: String,
    _description: String,
    _password: String,
    _max_users: i32,
    _floor_plan: String,
    _door_x: i32,
    _door_y: i32,
    _walk_through: bool,
    _floor_thickness: i32,
    _wall_thickness: i32,
    _wall_height: i32,
    _wall_hidden: bool,
}

pub struct Room {/* pub data: Option<RoomData>, */}

impl Room {
    pub fn new() -> Room {
        Room {}
    }
}

impl Actor for Room {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Room process {} started", "-1");
        System::current().stop(); // <- stop system
    }
}
