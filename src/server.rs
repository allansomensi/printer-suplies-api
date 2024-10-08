use std::{env, sync::Arc};

use sqlx::PgPool;
use tracing::{error, info};

use crate::{models::database::AppState, router};

pub async fn run() -> Result<(), axum::Error> {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = match PgPool::connect(&database_url).await {
        Ok(pool) => {
            info!("✅ Conectado ao banco de dados");
            pool
        }
        Err(e) => {
            error!("❌ Erro ao se conectar ao banco de dados: {e}");
            std::process::exit(1);
        }
    };

    let app = router::routes(Arc::new(AppState { db: pool.clone() }));

    let addr = env::var("HOST").expect("Erro ao carregar env HOST");
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => {
            info!("✅ Servidor iniciado em {}", &addr);
            listener
        }
        Err(e) => {
            error!("❌ Erro ao iniciar o servidor: {e}");
            std::process::exit(1)
        }
    };

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
