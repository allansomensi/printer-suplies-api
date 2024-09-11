use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing::{error, info};
use uuid::Uuid;

use crate::models::{
    brand::{Brand, CreateBrandRequest, DeleteBrandRequest, UpdateBrandRequest},
    database::AppState,
};

pub async fn count_brands(State(state): State<Arc<AppState>>) -> Json<i32> {
    let brand_count: Result<(i32,), sqlx::Error> =
        sqlx::query_as(r#"SELECT COUNT(*)::int FROM brands"#)
            .fetch_one(&state.db)
            .await;

    match brand_count {
        Ok((count,)) => {
            info!("Successfully retrieved brand count: {}", count);
            Json(count)
        }
        Err(e) => {
            error!("Error retrieving brand count: {e}");
            Json(0)
        }
    }
}

pub async fn search_brand(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Json<Option<Brand>> {
    match sqlx::query_as::<_, Brand>(r#"SELECT * FROM brands WHERE id = $1;"#)
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(brand) => {
            info!("Brand found: {id}");
            Json(Some(brand))
        }
        Err(e) => {
            error!("Error retrieving brand: {e}");
            Json(None)
        }
    }
}

pub async fn show_brands(State(state): State<Arc<AppState>>) -> Json<Vec<Brand>> {
    match sqlx::query_as(r#"SELECT * FROM brands"#)
        .fetch_all(&state.db)
        .await
    {
        Ok(brands) => {
            info!("Brands listed successfully");
            Json(brands)
        }
        Err(e) => {
            error!("Error listing brands: {e}");
            Json(Vec::new())
        }
    }
}

pub async fn create_brand(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateBrandRequest>,
) -> impl IntoResponse {
    let new_brand = Brand::new(&request.name);

    match sqlx::query(
        r#"
        INSERT INTO brands (id, name)
        VALUES ($1, $2)
        "#,
    )
    .bind(new_brand.id)
    .bind(new_brand.name)
    .execute(&state.db)
    .await
    {
        Ok(_) => {
            info!("Brand created! ID: {}", &new_brand.id);
            StatusCode::CREATED
        }
        Err(e) => {
            info!("Error creating brand: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn update_brand(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateBrandRequest>,
) -> impl IntoResponse {
    let brand_id = request.id;
    let new_name = request.name;

    match sqlx::query(r#"UPDATE brands SET name = $1 WHERE id = $2"#)
        .bind(&new_name)
        .bind(brand_id)
        .execute(&state.db)
        .await
    {
        Ok(_) => {
            info!("Brand updated! ID: {}", &brand_id);
            StatusCode::OK
        }
        Err(e) => {
            info!("Error updating brand: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn delete_brand(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteBrandRequest>,
) -> impl IntoResponse {
    match sqlx::query(r#"DELETE FROM brands WHERE id = $1"#)
        .bind(request.id)
        .execute(&state.db)
        .await
    {
        Ok(_) => {
            info!("Brand deleted! ID: {}", &request.id);
            StatusCode::OK
        }
        Err(e) => {
            info!("Error deleting brand: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
