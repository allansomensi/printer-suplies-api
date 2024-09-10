use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    models::toner::{CreateTonerRequest, DeleteTonerRequest, Toner},
    AppState,
};

pub async fn show_toners(State(state): State<Arc<AppState>>) -> Json<Vec<Toner>> {
    let row: Vec<Toner> = sqlx::query_as("SELECT * FROM toners")
        .fetch_all(&state.db)
        .await
        .unwrap();
    Json(row)
}

pub async fn count_toners(State(state): State<Arc<AppState>>) -> Json<i32> {
    let row: (i32,) = sqlx::query_as("SELECT COUNT(*)::int FROM toners")
        .fetch_one(&state.db)
        .await
        .unwrap();
    Json(row.0)
}

pub async fn create_toner(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateTonerRequest>,
) -> impl IntoResponse {
    let new_toner = Toner::new(&request.name, &request.color);

    match sqlx::query(
        "
        INSERT INTO toners (id, name, color)
        VALUES ($1, $2, $3)
        ",
    )
    .bind(new_toner.id)
    .bind(&new_toner.name)
    .bind(&new_toner.color)
    .execute(&state.db)
    .await
    {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn delete_toner(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteTonerRequest>,
) -> impl IntoResponse {
    match sqlx::query("DELETE FROM toners WHERE id = $1")
        .bind(request.id)
        .execute(&state.db)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
