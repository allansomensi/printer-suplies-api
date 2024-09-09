use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;

use crate::models::brand::{Brand, CreateBrandRequest, DeleteBrandRequest};

pub async fn show_brands(State(pool): State<PgPool>) -> Json<Vec<Brand>> {
    let row: Vec<Brand> = sqlx::query_as("SELECT * FROM brands")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(row)
}

pub async fn count_brands(State(pool): State<PgPool>) -> Json<i32> {
    let row: (i32,) = sqlx::query_as("SELECT COUNT(*)::int FROM brands")
        .fetch_one(&pool)
        .await
        .unwrap();
    Json(row.0)
}

pub async fn create_brand(
    State(pool): State<PgPool>,
    Json(request): Json<CreateBrandRequest>,
) -> impl IntoResponse {
    let new_brand = Brand::new(&request.name);

    match sqlx::query(
        "
        INSERT INTO brands (id, name)
        VALUES ($1, $2)
        ",
    )
    .bind(new_brand.id)
    .bind(&new_brand.name)
    .execute(&pool)
    .await
    {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn delete_brand(
    State(pool): State<PgPool>,
    Json(request): Json<DeleteBrandRequest>,
) -> impl IntoResponse {
    match sqlx::query("DELETE FROM brands WHERE id = $1")
        .bind(request.id)
        .execute(&pool)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
