use std::env;

use axum::{extract::State, response::IntoResponse, Json};
use chrono::Utc;
use sqlx::PgPool;

use crate::models::status::{Database, Dependencies, Status};

pub async fn show_status(State(pool): State<PgPool>) -> impl IntoResponse {
    let db_version: (String,) = sqlx::query_as(r#"SHOW server_version;"#)
        .fetch_one(&pool)
        .await
        .unwrap();

    let max_connections: (String,) = sqlx::query_as("SHOW max_connections;")
        .fetch_one(&pool)
        .await
        .unwrap();
    let max_connections: u16 = max_connections.0.parse().unwrap();

    let opened_connections: (i32,) =
        sqlx::query_as(r#"SELECT count(*)::int FROM pg_stat_activity WHERE datname = $1;"#)
            .bind(env::var("POSTGRES_DB").unwrap())
            .fetch_one(&pool)
            .await
            .unwrap();
    let opened_connections: u16 = opened_connections.0 as u16;

    let database = Database {
        version: db_version.0,
        max_connections,
        opened_connections,
    };

    let status = Status {
        updated_at: Utc::now(),
        dependencies: Dependencies { database },
    };

    Json(status)
}
