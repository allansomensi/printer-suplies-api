use chrono::Utc;
use reqwest::StatusCode;
use sqlx::Row;
use std::sync::Arc;
use tracing::{error, info};

use axum::{extract::State, response::IntoResponse, Json};
use uuid::Uuid;

use crate::models::{
    database::AppState,
    movement::{CreateMovementRequest, Movement},
};

pub async fn show_movements(State(state): State<Arc<AppState>>) -> Json<Vec<Movement>> {
    match sqlx::query_as(r#"SELECT * FROM movements"#)
        .fetch_all(&state.db)
        .await
    {
        Ok(movements) => {
            info!("Movements listed successfully");
            Json(movements)
        }
        Err(e) => {
            error!("Error listing movements: {e}");
            Json(Vec::new())
        }
    }
}

pub async fn create_movement(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateMovementRequest>,
) -> impl IntoResponse {
    match request.toner_id {
        Some(_) => {
            let printer_id = sqlx::query("SELECT id FROM printers WHERE toner = $1")
                .bind(&request.toner_id)
                .fetch_one(&state.db)
                .await;

            let printer_id: Uuid = printer_id.unwrap().try_get("id").unwrap();

            let new_movement = Movement::new(printer_id, request.toner_id, None, request.quantity);

            sqlx::query(
                r#"
                INSERT INTO movements (id, printer_id, toner_id, drum_id, quantity, created_at)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(printer_id)
            .bind(new_movement.toner_id)
            .bind(new_movement.drum_id)
            .bind(new_movement.quantity)
            .bind(Utc::now())
            .execute(&state.db)
            .await
            .unwrap();

            sqlx::query("UPDATE toners SET stock = stock + $1 WHERE id = $2")
                .bind(request.quantity)
                .bind(request.toner_id)
                .execute(&state.db)
                .await
                .unwrap();

            Ok(StatusCode::CREATED)
        }
        None => Err(StatusCode::INTERNAL_SERVER_ERROR), // TODO! Toner movement
    }
}
