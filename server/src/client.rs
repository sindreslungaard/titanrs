use tokio::sync::mpsc;

pub struct Client {
    id: usize,
    player: Option<mpsc::Sender<player::command::Command>>
}

