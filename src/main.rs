use axum::{routing::get, Router};
use handlers::status;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    dotenvy::from_filename(".env.development").unwrap();

    let database_url = std::env::var("DATABASE_URL").unwrap();

    println!("🌟 Printer Supplies API 🌟");

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Conectado ao banco de dados");
            pool
        }
        Err(e) => {
            eprintln!("❌ Erro ao se conectar ao banco de dados: {e}");
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/status", get(status::show_status))
        .with_state(pool);

    let addr = env::var("HOST").expect("Erro ao carregar env HOST");
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => {
            println!("✅ Servidor iniciado em {}", &addr);
            listener
        }
        Err(e) => {
            eprintln!("❌ Erro ao iniciar o servidor: {e}");
            std::process::exit(1)
        }
    };

    axum::serve(listener, app).await.unwrap();
}
