use crate::core::{Clients, State};
use crate::net::client::*;

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{
    ws::{Message, WebSocket},
    Filter,
};

static ID_GENERATOR: AtomicUsize = AtomicUsize::new(1);

pub async fn start(state: Arc<State>) {
    let clients = state.clients.clone();
    // Turn our "state" into a new Filter...
    let filter = warp::any().map(move || clients.clone());

    // GET /chat -> websocket upgrade
    let ws = warp::path::end()
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .and(filter)
        .map(|ws: warp::ws::Ws, clients| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| client_connected(socket, clients))
        });

    println!("Listening on 127.0.0.1:3030");

    warp::serve(ws).run(([127, 0, 0, 1], 3030)).await;
}

async fn client_connected(ws: WebSocket, clients: Clients) {
    // Use a counter to assign a new unique ID for this user.
    let my_id = ID_GENERATOR.fetch_add(1, Ordering::Relaxed);

    println!("new chat user: {}", my_id);

    // Split the socket into a sender and receive of messages.
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });

    let client = Client::new(my_id, tx);

    // Save the sender in our list of connected users.
    clients.write().await.insert(my_id, client);

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.

    // Every time the user sends a message, broadcast it to
    // all other users...
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", my_id, e);
                break;
            }
        };

        match clients.read().await.get(&my_id) {
            Some(client) => match &client.user {
                Some(_user) => {}
                None => {
                    println!("we need to authenticate the user");
                }
            },
            None => {
                println!("error parsing client message")
            }
        }
        match clients.read().await.get(&my_id) {
            Some(client) => client.receive(msg),
            None => {}
        }
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    client_disconnected(my_id, &clients).await;
}

async fn client_disconnected(id: usize, clients: &Clients) {
    eprintln!("good bye user: {}", id);

    // Stream closed up, so remove from the user list
    clients.write().await.remove(&id);
}
