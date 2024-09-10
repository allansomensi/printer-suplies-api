use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    models::drum::{CreateDrumRequest, DeleteDrumRequest, Drum},
    AppState,
};

pub async fn show_drums(State(state): State<Arc<AppState>>) -> Json<Vec<Drum>> {
    let row: Vec<Drum> = sqlx::query_as("SELECT * FROM drums")
        .fetch_all(&state.db)
        .await
        .unwrap();
    Json(row)
}

pub async fn count_drums(State(state): State<Arc<AppState>>) -> Json<i32> {
    let row: (i32,) = sqlx::query_as("SELECT COUNT(*)::int FROM drums")
        .fetch_one(&state.db)
        .await
        .unwrap();
    Json(row.0)
}

pub async fn create_drum(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateDrumRequest>,
) -> impl IntoResponse {
    let new_drum = Drum::new(&request.name);

    match sqlx::query(
        "
        INSERT INTO drums (id, name)
        VALUES ($1, $2)
        ",
    )
    .bind(new_drum.id)
    .bind(&new_drum.name)
    .execute(&state.db)
    .await
    {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn delete_drum(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteDrumRequest>,
) -> impl IntoResponse {
    match sqlx::query("DELETE FROM drums WHERE id = $1")
        .bind(request.id)
        .execute(&state.db)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
