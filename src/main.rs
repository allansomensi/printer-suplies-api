mod handlers;
mod models;
mod router;
mod server;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt().init();

    println!("🌟 Printer Supplies API 🌟");

    server::run().await.unwrap();
}
