use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::models::{
    database::AppState,
    supplies::toner::{CreateTonerRequest, DeleteTonerRequest, Toner, UpdateTonerRequest},
};

pub async fn count_toners(State(state): State<Arc<AppState>>) -> Json<i32> {
    let row: (i32,) = sqlx::query_as(r#"SELECT COUNT(*)::int FROM toners"#)
        .fetch_one(&state.db)
        .await
        .unwrap();
    Json(row.0)
}

pub async fn search_toner(Path(id): Path<Uuid>, State(state): State<Arc<AppState>>) -> Json<Toner> {
    Json(
        sqlx::query_as(r#"SELECT * FROM toners WHERE id = $1;"#)
            .bind(id)
            .fetch_one(&state.db)
            .await
            .unwrap(),
    )
}

pub async fn show_toners(State(state): State<Arc<AppState>>) -> Json<Vec<Toner>> {
    Json(
        sqlx::query_as(r#"SELECT * FROM toners"#)
            .fetch_all(&state.db)
            .await
            .unwrap(),
    )
}

pub async fn create_toner(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateTonerRequest>,
) -> impl IntoResponse {
    let new_toner = Toner::new(&request.name, request.color);

    match sqlx::query(
        r#"
        INSERT INTO toners (id, name, color)
        VALUES ($1, $2, $3)
        "#,
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

pub async fn update_toner(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateTonerRequest>,
) -> impl IntoResponse {
    let toner_id = request.id;
    let new_name = request.name;
    let new_color = request.color;

    match sqlx::query(r#"UPDATE toners SET name = $1, color = $2 WHERE id = $3"#)
        .bind(&new_name)
        .bind(&new_color)
        .bind(toner_id)
        .execute(&state.db)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn delete_toner(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteTonerRequest>,
) -> impl IntoResponse {
    match sqlx::query(r#"DELETE FROM toners WHERE id = $1"#)
        .bind(request.id)
        .execute(&state.db)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
