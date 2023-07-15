use sqlx::mysql::MySqlPool;
use std::collections::HashMap;
use tokio::sync::oneshot;
use tokio::sync::mpsc;
use ctx::player::*;
use anyhow::anyhow;

use crate::Player;

pub struct Service {
    ctx: ctx::Context,
    players: HashMap<i32, Player>,
}

pub fn start(ctx: ctx::Context, rx: mpsc::Receiver<Command>) {
    tokio::spawn(async move {
        Service {
            ctx,
            players: HashMap::new(),
        }.run(rx).await;
    });

    info!("Player service started");
}

impl Service {
    async fn run(&self, mut rx: mpsc::Receiver<Command>) {

        while let Some(msg) = rx.recv().await {
            match msg {
                Command::TryAuthenticate { player_id, access_token, response } => {
                    todo!();
                }
            };
        }
    }
}

