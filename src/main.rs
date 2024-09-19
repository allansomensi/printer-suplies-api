use tracing::{error, info};

mod config;
mod handlers;
mod logger;
mod models;
mod router;
mod server;

#[tokio::main]
async fn main() {
    println!("🌟 Printer Supplies API 🌟");

    logger::init();
    match config::Config::init() {
        Ok(_) => {
            info!("✅ Configurações carregadas!");
        }
        Err(e) => {
            error!("Não foi possível carregar as configurações: {:?}", e);
            std::process::exit(1);
        }
    }
    server::run().await.unwrap();
}
