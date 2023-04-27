use std::sync::Arc;

use crate::core::State;

mod core;
mod net;

#[tokio::main]
async fn main() {
    println!("Starting...");

    let state = State::new();

    print_users_cron(state.clone());

    net::server::start(state.clone()).await;
}

fn print_users_cron(state: Arc<State>) {
    tokio::spawn(async move {
        loop {
            let clients = state.clients.read().await.len();
            println!("Clients connected: {}", clients);
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    });
}
