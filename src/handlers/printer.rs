use std::{str::FromStr, sync::Arc};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use uuid::Uuid;

use crate::models::{
    database::AppState,
    printer::{CreatePrinterRequest, DeletePrinterRequest, Printer},
};

pub async fn show_printers(State(state): State<Arc<AppState>>) -> Json<Vec<Printer>> {
    let row: Vec<Printer> = sqlx::query_as(r#"SELECT * FROM printers"#)
        .fetch_all(&state.db)
        .await
        .unwrap();
    Json(row)
}

pub async fn count_printers(State(state): State<Arc<AppState>>) -> Json<i32> {
    let row: (i32,) = sqlx::query_as(r#"SELECT COUNT(*)::int FROM printers"#)
        .fetch_one(&state.db)
        .await
        .unwrap();
    Json(row.0)
}

pub async fn create_printer(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreatePrinterRequest>,
) -> impl IntoResponse {
    let new_printer = Printer::new(
        &request.name,
        &request.model,
        Uuid::from_str(&request.brand).unwrap(),
        Uuid::from_str(&request.toner).unwrap(),
        Uuid::from_str(&request.drum).unwrap(),
    );

    match sqlx::query(
        r#"
        INSERT INTO printers (id, name, model, brand, toner, drum)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(new_printer.id)
    .bind(new_printer.name)
    .bind(new_printer.model)
    .bind(new_printer.brand)
    .bind(new_printer.toner)
    .bind(new_printer.drum)
    .execute(&state.db)
    .await
    {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn delete_printer(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeletePrinterRequest>,
) -> impl IntoResponse {
    match sqlx::query(r#"DELETE FROM printers WHERE id = $1"#)
        .bind(request.id)
        .execute(&state.db)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
