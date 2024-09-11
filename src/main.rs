mod config;
mod handlers;
mod models;
mod router;
mod server;

#[tokio::main]
async fn main() {
    println!("🌟 Printer Supplies API 🌟");

    config::init();
    server::run().await.unwrap();
}
