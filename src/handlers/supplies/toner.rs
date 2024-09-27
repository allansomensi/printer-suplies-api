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
    supplies::toner::{CreateTonerRequest, Toner, UpdateTonerRequest},
    DeleteRequest,
};

pub async fn count_toners(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let toner_count: Result<(i32,), sqlx::Error> =
        sqlx::query_as(r#"SELECT COUNT(*)::int FROM toners;"#)
            .fetch_one(&state.db)
            .await;
    match toner_count {
        Ok((count,)) => {
            info!("Successfully retrieved toner count: {}", count);
            Ok(Json(count))
        }
        Err(e) => {
            error!("Error retrieving toner count: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error retrieving toner count."),
            ))
        }
    }
}

pub async fn search_toner(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Toner>(r#"SELECT * FROM toners WHERE id = $1;"#)
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(toner)) => {
            info!("Toner found: {id}");
            (StatusCode::OK, Json(Some(toner)))
        }
        Ok(None) => {
            error!("No toner found.");
            (StatusCode::NOT_FOUND, Json(None))
        }
        Err(e) => {
            error!("Error retrieving toner: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
        }
    }
}

pub async fn show_toners(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let toners: Result<Vec<Toner>, sqlx::Error> = sqlx::query_as(r#"SELECT * FROM toners;"#)
        .fetch_all(&state.db)
        .await;
    match toners {
        Ok(toners) => {
            info!("Toners listed successfully");
            Ok(Json(toners))
        }
        Err(e) => {
            error!("Error listing toners: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error listing toners."),
            ))
        }
    }
}

pub async fn create_toner(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateTonerRequest>,
) -> impl IntoResponse {
    let new_toner = Toner::new(&request.name);

    // Check duplicate
    match sqlx::query(r#"SELECT id FROM toners WHERE name = $1;"#)
        .bind(&new_toner.name)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            error!("Toner '{}' already exists.", &new_toner.name);
            return (StatusCode::CONFLICT, Err(Json("Toner already exists.")));
        }
        Ok(None) => {
            // Name is empty
            if new_toner.name.is_empty() {
                error!("Toner name cannot be empty.");
                return (
                    StatusCode::BAD_REQUEST,
                    Err(Json("Toner name cannot be empty.")),
                );
            }

            // Name too short
            if new_toner.name.len() < 4 {
                error!("Toner name is too short.");
                return (
                    StatusCode::BAD_REQUEST,
                    Err(Json("Toner name is too short.")),
                );
            }

            // Name too long
            if new_toner.name.len() > 20 {
                error!("Toner name is too long.");
                return (
                    StatusCode::BAD_REQUEST,
                    Err(Json("Toner name is too long.")),
                );
            }

            match sqlx::query(r#"INSERT INTO toners (id, name) VALUES ($1, $2);"#)
                .bind(new_toner.id)
                .bind(&new_toner.name)
                .execute(&state.db)
                .await
            {
                Ok(_) => {
                    info!("Toner created! ID: {}", &new_toner.id);
                    (StatusCode::CREATED, Ok(Json(new_toner.id)))
                }
                Err(e) => {
                    error!("Error creating toner: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Err(Json("Error creating toner.")),
                    )
                }
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Err(Json("Error creating toner.")),
        ),
    }
}

pub async fn update_toner(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateTonerRequest>,
) -> impl IntoResponse {
    let toner_id = request.id;
    let new_name = request.name;

    // ID not found
    match sqlx::query(r#"SELECT id FROM toners WHERE id = $1;"#)
        .bind(toner_id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            // Name is empty
            if new_name.is_empty() {
                error!("Toner name cannot be empty.");
                return (
                    StatusCode::CONFLICT,
                    Err(Json("Toner name cannot be empty.")),
                );
            }

            // Name too short
            if new_name.len() < 4 {
                error!("Toner name is too short.");
                return (
                    StatusCode::BAD_REQUEST,
                    Err(Json("Toner name is too short.")),
                );
            }

            // Name too long
            if new_name.len() > 20 {
                error!("Toner name is too long.");
                return (
                    StatusCode::BAD_REQUEST,
                    Err(Json("Toner name is too long.")),
                );
            }

            // Check duplicate
            match sqlx::query(r#"SELECT id FROM toners WHERE name = $1 AND id != $2;"#)
                .bind(&new_name)
                .bind(toner_id)
                .fetch_optional(&state.db)
                .await
            {
                Ok(Some(_)) => {
                    error!("Toner name already exists.");
                    (
                        StatusCode::CONFLICT,
                        Err(Json("Toner name already exists.")),
                    )
                }
                Ok(None) => {
                    match sqlx::query(r#"UPDATE toners SET name = $1 WHERE id = $2;"#)
                        .bind(&new_name)
                        .bind(toner_id)
                        .execute(&state.db)
                        .await
                    {
                        Ok(_) => {
                            info!("Toner updated! ID: {}", &toner_id);
                            (StatusCode::OK, Ok(Json(toner_id)))
                        }
                        Err(e) => {
                            error!("Error updating toner: {}", e);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Err(Json("Error updating toner.")),
                            )
                        }
                    }
                }
                Err(e) => {
                    error!("Error checking for duplicate toner name: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Err(Json("Error checking for duplicated toner name.")),
                    )
                }
            }
        }
        Ok(None) => {
            error!("Toner ID not found.");
            (StatusCode::NOT_FOUND, Err(Json("Toner ID not found.")))
        }
        Err(e) => {
            error!("Error fetching toner by ID: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Err(Json("Error fetching toner by ID")),
            )
        }
    }
}

pub async fn delete_toner(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteRequest>,
) -> impl IntoResponse {
    match sqlx::query(r#"SELECT id FROM toners WHERE id = $1;"#)
        .bind(request.id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            match sqlx::query(r#"DELETE FROM toners WHERE id = $1;"#)
                .bind(request.id)
                .execute(&state.db)
                .await
            {
                Ok(_) => {
                    info!("Toner deleted! ID: {}", &request.id);
                    (StatusCode::OK, Ok(Json("Toner deleted!")))
                }
                Err(e) => {
                    error!("Error deleting toner: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Ok(Json("Error deleting toner.")),
                    )
                }
            }
        }
        Ok(None) => {
            error!("Toner ID not found.");
            (StatusCode::NOT_FOUND, Err(Json("Toner ID not found")))
        }
        Err(e) => {
            error!("Error deleting toner: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Err(Json("Error deleting drum.")),
            )
        }
    }
}
