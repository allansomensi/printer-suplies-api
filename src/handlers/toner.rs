use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;

use crate::models::toner::{CreateTonerRequest, DeleteTonerRequest, Toner};

pub async fn show_toners(State(pool): State<PgPool>) -> Json<Vec<Toner>> {
    let row: Vec<Toner> = sqlx::query_as("SELECT * FROM toners")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(row)
}

pub async fn count_toners(State(pool): State<PgPool>) -> Json<i32> {
    let row: (i32,) = sqlx::query_as("SELECT COUNT(*)::int FROM toners")
        .fetch_one(&pool)
        .await
        .unwrap();
    Json(row.0)
}

pub async fn create_toner(
    State(pool): State<PgPool>,
    Json(request): Json<CreateTonerRequest>,
) -> impl IntoResponse {
    let new_ticket = Toner::new(&request.name, &request.color);

    match sqlx::query(
        "
        INSERT INTO toners (id, name, color)
        VALUES ($1, $2, $3)
        ",
    )
    .bind(new_ticket.id)
    .bind(&new_ticket.name)
    .bind(&new_ticket.color)
    .execute(&pool)
    .await
    {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn delete_toner(
    State(pool): State<PgPool>,
    Json(request): Json<DeleteTonerRequest>,
) -> impl IntoResponse {
    match sqlx::query("DELETE FROM toners WHERE id = $1")
        .bind(request.id)
        .execute(&pool)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
