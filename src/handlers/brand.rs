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
) -> impl IntoResponse {
    match sqlx::query_as::<_, Brand>("SELECT * FROM brands WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(brand)) => {
            info!("Brand found: {id}");
            (StatusCode::OK, Json(Some(brand)))
        }
        Ok(None) => {
            error!("No brand found.");
            (StatusCode::NOT_FOUND, Json(None))
        }
        Err(e) => {
            error!("Error retrieving brand: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
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

    // Check duplicate
    match sqlx::query("SELECT id FROM brands WHERE name = $1")
        .bind(&new_brand.name)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            error!("Brand '{}' already exists.", &new_brand.name);
            StatusCode::CONFLICT
        }
        Ok(None) => {
            // Name is empty
            if new_brand.name.is_empty() {
                error!("Brand name cannot be empty.");
                return StatusCode::BAD_REQUEST;
            }

            // Name too short
            if new_brand.name.len() < 4 {
                error!("Brand name is too short.");
                return StatusCode::BAD_REQUEST;
            }

            // Name too long
            if new_brand.name.len() > 20 {
                error!("Brand name is too long.");
                return StatusCode::BAD_REQUEST;
            }

            match sqlx::query(
                r#"
                INSERT INTO brands (id, name)
                VALUES ($1, $2)
                "#,
            )
            .bind(new_brand.id)
            .bind(&new_brand.name)
            .execute(&state.db)
            .await
            {
                Ok(_) => {
                    info!("Brand created! ID: {}", &new_brand.id);
                    StatusCode::CREATED
                }
                Err(e) => {
                    error!("Error creating brand: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn update_brand(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateBrandRequest>,
) -> impl IntoResponse {
    let brand_id = request.id;
    let new_name = request.name;

    // ID not found
    match sqlx::query(r#"SELECT id FROM brands WHERE id = $1"#)
        .bind(brand_id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            // Name is empty
            if new_name.is_empty() {
                error!("Brand name cannot be empty.");
                return StatusCode::BAD_REQUEST;
            }

            // Name too short
            if new_name.len() < 4 {
                error!("Brand name is too short.");
                return StatusCode::BAD_REQUEST;
            }

            // Name too long
            if new_name.len() > 20 {
                error!("Brand name is too long.");
                return StatusCode::BAD_REQUEST;
            }

            // Check duplicate
            match sqlx::query(r#"SELECT id FROM brands WHERE name = $1 AND id != $2"#)
                .bind(&new_name)
                .bind(brand_id)
                .fetch_optional(&state.db)
                .await
            {
                Ok(Some(_)) => {
                    error!("Brand name already exists.");
                    return StatusCode::BAD_REQUEST;
                }
                Ok(None) => {
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
                            error!("Error updating brand: {}", e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        }
                    }
                }
                Err(e) => {
                    error!("Error checking for duplicate brand name: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
        }
        Ok(None) => {
            error!("Brand ID not found.");
            StatusCode::NOT_FOUND
        }
        Err(e) => {
            error!("Error fetching brand by ID: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn delete_brand(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteBrandRequest>,
) -> impl IntoResponse {
    match sqlx::query(r#"SELECT id FROM brands WHERE id = $1"#)
        .bind(request.id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
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
                    error!("Error deleting brand: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
        }
        Ok(None) => {
            error!("Brand ID not found.");
            StatusCode::NOT_FOUND
        }
        Err(e) => {
            error!("Error deleting brand: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
