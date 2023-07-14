use tokio::sync::mpsc;

struct Client {
    id: usize,
    player: Option<mpsc::Sender<player::command::Command>>
}

