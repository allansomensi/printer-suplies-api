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
    database::AppState,
    supplies::toner::{CreateTonerRequest, DeleteTonerRequest, Toner, UpdateTonerRequest},
};

pub async fn count_toners(State(state): State<Arc<AppState>>) -> Json<i32> {
    let toner_count: Result<(i32,), sqlx::Error> =
        sqlx::query_as(r#"SELECT COUNT(*)::int FROM toners"#)
            .fetch_one(&state.db)
            .await;
    match toner_count {
        Ok((count,)) => {
            info!("Successfully retrieved toner count: {}", count);
            Json(count)
        }
        Err(e) => {
            error!("Error retrieving toner count: {e}");
            Json(0)
        }
    }
}

pub async fn search_toner(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Json<Option<Toner>> {
    match sqlx::query_as::<_, Toner>("SELECT * FROM toners WHERE id = $1;")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(toner) => {
            info!("Toner found: {id}");
            Json(Some(toner))
        }
        Err(e) => {
            error!("Error retrieving toner: {e}");
            Json(None)
        }
    }
}

pub async fn show_toners(State(state): State<Arc<AppState>>) -> Json<Vec<Toner>> {
    match sqlx::query_as(r#"SELECT * FROM toners"#)
        .fetch_all(&state.db)
        .await
    {
        Ok(toners) => {
            info!("Toners listed successfully");
            Json(toners)
        }
        Err(e) => {
            error!("Error listing toners: {e}");
            Json(Vec::new())
        }
    }
}

pub async fn create_toner(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateTonerRequest>,
) -> impl IntoResponse {
    let new_toner = Toner::new(&request.name);

    // Check duplicate
    match sqlx::query("SELECT id FROM toners WHERE name = $1")
        .bind(&new_toner.name)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            error!("Toner '{}' already exists.", &new_toner.name);
            StatusCode::CONFLICT
        }
        Ok(None) => {
            // Name is empty
            if new_toner.name.is_empty() {
                error!("Toner name cannot be empty.");
                return StatusCode::BAD_REQUEST;
            }

            // Name too short
            if new_toner.name.len() < 4 {
                error!("Toner name is too short.");
                return StatusCode::BAD_REQUEST;
            }

            // Name too long
            if new_toner.name.len() > 20 {
                error!("Toner name is too long.");
                return StatusCode::BAD_REQUEST;
            }

            match sqlx::query(
                r#"
                INSERT INTO toners (id, name)
                VALUES ($1, $2)
                "#,
            )
            .bind(new_toner.id)
            .bind(&new_toner.name)
            .execute(&state.db)
            .await
            {
                Ok(_) => {
                    info!("Toner created! ID: {}", &new_toner.id);
                    StatusCode::CREATED
                }
                Err(e) => {
                    error!("Error creating toner: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn update_toner(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateTonerRequest>,
) -> impl IntoResponse {
    let toner_id = request.id;
    let new_name = request.name;

    match sqlx::query(r#"UPDATE toners SET name = $1, color = $2 WHERE id = $3"#)
        .bind(&new_name)
        .bind(toner_id)
        .execute(&state.db)
        .await
    {
        Ok(_) => {
            info!("Toner updated! ID: {}", &toner_id);
            StatusCode::OK
        }
        Err(e) => {
            info!("Error updating toner: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
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
        Ok(_) => {
            info!("Toner deleted! ID: {}", &request.id);
            StatusCode::OK
        }
        Err(e) => {
            info!("Error deleting toner: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
