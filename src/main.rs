use axum::{routing::get, Router};
use handlers::{brand, drum, printer, status, toner};
use sqlx::PgPool;
use std::{env, sync::Arc};

mod handlers;
mod models;

struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap();

    println!("🌟 Printer Supplies API 🌟");

    let pool = match PgPool::connect(&database_url).await {
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
        // Status
        .route("/api/v1/status", get(status::show_status))
        // Printer
        .route(
            "/api/v1/printers",
            get(printer::show_printers)
                .post(printer::create_printer)
                .delete(printer::delete_printer),
        )
        .route("/api/v1/printer-count", get(printer::count_printers))
        // Toner
        .route(
            "/api/v1/toners",
            get(toner::show_toners)
                .post(toner::create_toner)
                .delete(toner::delete_toner),
        )
        .route("/api/v1/toner-count", get(toner::count_toners))
        // Drum
        .route(
            "/api/v1/drums",
            get(drum::show_drums)
                .post(drum::create_drum)
                .delete(drum::delete_drum),
        )
        .route("/api/v1/drum-count", get(drum::count_drums))
        // Brand
        .route(
            "/api/v1/brands",
            get(brand::show_brands)
                .post(brand::create_brand)
                .delete(brand::delete_brand),
        )
        .route("/api/v1/brand-count", get(brand::count_brands))
        .with_state(Arc::new(AppState { db: pool.clone() }));

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
