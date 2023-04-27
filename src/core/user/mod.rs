#[derive(Clone)]
pub struct User {
    id: usize,
    room_id: usize,
}

impl User {
    pub fn _new(id: usize) -> User {
        User { id, room_id: 0 }
    }

    pub fn _parse_msg(msg: String) {
        print!("Parsed msg {}", msg)
    }
}
