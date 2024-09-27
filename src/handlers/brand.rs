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
    brand::{Brand, CreateBrandRequest, UpdateBrandRequest},
    database::AppState,
    DeleteRequest,
};

pub async fn count_brands(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let brand_count: Result<(i32,), sqlx::Error> =
        sqlx::query_as(r#"SELECT COUNT(*)::int FROM brands;"#)
            .fetch_one(&state.db)
            .await;

    match brand_count {
        Ok((count,)) => {
            info!("Successfully retrieved brand count: {}", count);
            Ok(Json(count))
        }
        Err(e) => {
            error!("Error retrieving brand count: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error retrieving brand count."),
            ))
        }
    }
}

pub async fn search_brand(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Brand>(r#"SELECT * FROM brands WHERE id = $1;"#)
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

pub async fn show_brands(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let brands: Result<Vec<Brand>, sqlx::Error> = sqlx::query_as(r#"SELECT * FROM brands;"#)
        .fetch_all(&state.db)
        .await;
    match brands {
        Ok(brands) => {
            info!("Brands listed successfully");
            Ok(Json(brands))
        }
        Err(e) => {
            error!("Error listing brands: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error listing brands."),
            ))
        }
    }
}

pub async fn create_brand(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateBrandRequest>,
) -> impl IntoResponse {
    let new_brand = Brand::new(&request.name);

    // Check duplicate
    match sqlx::query(r#"SELECT id FROM brands WHERE name = $1;"#)
        .bind(&new_brand.name)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            error!("Brand '{}' already exists.", &new_brand.name);
            (StatusCode::CONFLICT, Err(Json("Brand already exists.")))
        }
        Ok(None) => {
            // Name is empty
            if new_brand.name.is_empty() {
                error!("Brand name cannot be empty.");
                return (
                    StatusCode::BAD_REQUEST,
                    Err(Json("Brand name cannot be empty.")),
                );
            }

            // Name too short
            if new_brand.name.len() < 4 {
                error!("Brand name is too short.");
                return (
                    StatusCode::BAD_REQUEST,
                    Err(Json("Brand name is too short.")),
                );
            }

            // Name too long
            if new_brand.name.len() > 20 {
                error!("Brand name is too long.");
                return (StatusCode::BAD_REQUEST, Err(Json("Drum name is too long.")));
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
                    (StatusCode::CREATED, Ok(Json(new_brand.id)))
                }
                Err(e) => {
                    error!("Error creating brand: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Err(Json("Error creating brand.")),
                    )
                }
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Err(Json("Error creating brand.")),
        ),
    }
}

pub async fn update_brand(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateBrandRequest>,
) -> impl IntoResponse {
    let brand_id = request.id;
    let new_name = request.name;

    // ID not found
    match sqlx::query(r#"SELECT id FROM brands WHERE id = $1;"#)
        .bind(brand_id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            // Name is empty
            if new_name.is_empty() {
                error!("Brand name cannot be empty.");
                return (
                    StatusCode::BAD_REQUEST,
                    Err(Json("Brand name cannot be empty.")),
                );
            }

            // Name too short
            if new_name.len() < 4 {
                error!("Brand name is too short.");
                return (
                    StatusCode::BAD_REQUEST,
                    Err(Json("Brand name is too short.")),
                );
            }

            // Name too long
            if new_name.len() > 20 {
                error!("Brand name is too long.");
                return (
                    StatusCode::BAD_REQUEST,
                    Err(Json("Brand name is too long.")),
                );
            }

            // Check duplicate
            match sqlx::query(r#"SELECT id FROM brands WHERE name = $1 AND id != $2;"#)
                .bind(&new_name)
                .bind(brand_id)
                .fetch_optional(&state.db)
                .await
            {
                Ok(Some(_)) => {
                    error!("Brand name already exists.");
                    (StatusCode::CONFLICT, Err(Json("Brand already exists.")))
                }
                Ok(None) => {
                    match sqlx::query(r#"UPDATE brands SET name = $1 WHERE id = $2;"#)
                        .bind(&new_name)
                        .bind(brand_id)
                        .execute(&state.db)
                        .await
                    {
                        Ok(_) => {
                            info!("Brand updated! ID: {}", &brand_id);
                            (StatusCode::OK, Ok(Json(brand_id)))
                        }
                        Err(e) => {
                            error!("Error updating brand: {}", e);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Err(Json("Error updating drum.")),
                            )
                        }
                    }
                }
                Err(e) => {
                    error!("Error checking for duplicate brand name: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Err(Json("Error checking for duplicated brand name.")),
                    )
                }
            }
        }
        Ok(None) => {
            error!("Brand ID not found.");
            (StatusCode::NOT_FOUND, Err(Json("Brand ID not found.")))
        }
        Err(e) => {
            error!("Error fetching brand by ID: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Err(Json("Error fetching brand by ID")),
            )
        }
    }
}

pub async fn delete_brand(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteRequest>,
) -> impl IntoResponse {
    match sqlx::query(r#"SELECT id FROM brands WHERE id = $1;"#)
        .bind(request.id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            match sqlx::query(r#"DELETE FROM brands WHERE id = $1;"#)
                .bind(request.id)
                .execute(&state.db)
                .await
            {
                Ok(_) => {
                    info!("Brand deleted! ID: {}", &request.id);
                    (StatusCode::OK, Ok(Json("Brand deleted!")))
                }
                Err(e) => {
                    error!("Error deleting brand: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Ok(Json("Error deleting brand.")),
                    )
                }
            }
        }
        Ok(None) => {
            error!("Brand ID not found.");
            (StatusCode::NOT_FOUND, Err(Json("Brand ID not found")))
        }
        Err(e) => {
            error!("Error deleting brand: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Err(Json("Error deleting brand.")),
            )
        }
    }
}
