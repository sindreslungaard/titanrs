mod net;

#[tokio::main]
async fn main() {
    println!("Starting...");

    net::server::start().await;
}
