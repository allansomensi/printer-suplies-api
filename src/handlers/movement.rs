use chrono::Utc;
use reqwest::StatusCode;
use sqlx::Row;
use std::sync::Arc;
use tracing::{error, info};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::models::{
    database::AppState,
    movement::{
        CreateDrumMovementRequest, CreateTonerMovementRequest, Movement, UpdateMovementRequest,
    },
    DeleteRequest,
};

pub async fn count_all_movements(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let movement_count: Result<(i32,), sqlx::Error> =
        sqlx::query_as(r#"SELECT COUNT(*)::int FROM movements;"#)
            .fetch_one(&state.db)
            .await;

    match movement_count {
        Ok((count,)) => {
            info!("Successfully retrieved movement count: {}", count);
            Ok((StatusCode::OK, Json(count)))
        }
        Err(e) => {
            error!("Error retrieving movement count: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error retrieving movement count."),
            ))
        }
    }
}

pub async fn count_toner_movements(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let movement_count: Result<(i32,), sqlx::Error> =
        sqlx::query_as(r#"SELECT COUNT(*)::int FROM movements WHERE toner_id IS NOT NULL;"#)
            .fetch_one(&state.db)
            .await;

    match movement_count {
        Ok((count,)) => {
            info!("Successfully retrieved movement count: {}", count);
            Ok((StatusCode::OK, Json(count)))
        }
        Err(e) => {
            error!("Error retrieving toner movement count: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error retrieving toner movement count."),
            ))
        }
    }
}

pub async fn count_drum_movements(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let movement_count: Result<(i32,), sqlx::Error> =
        sqlx::query_as(r#"SELECT COUNT(*)::int FROM movements WHERE drum_id IS NOT NULL;"#)
            .fetch_one(&state.db)
            .await;

    match movement_count {
        Ok((count,)) => {
            info!("Successfully retrieved movement count: {}", count);
            Ok((StatusCode::OK, Json(count)))
        }
        Err(e) => {
            error!("Error retrieving movement count: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error retrieving drum movement count."),
            ))
        }
    }
}

pub async fn search_movement(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Movement>(r#"SELECT * FROM movements WHERE id = $1"#)
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(movement)) => {
            info!("Movement found: {id}");
            (StatusCode::OK, Json(Some(movement)))
        }
        Ok(None) => {
            error!("No movement found.");
            (StatusCode::NOT_FOUND, Json(None))
        }
        Err(e) => {
            error!("Error retrieving movement: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
        }
    }
}

pub async fn show_all_movements(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let movements: Result<Vec<Movement>, sqlx::Error> =
        sqlx::query_as(r#"SELECT * FROM movements"#)
            .fetch_all(&state.db)
            .await;
    match movements {
        Ok(movements) => {
            info!("Movements listed successfully");
            Ok((StatusCode::OK, Json(movements)))
        }
        Err(e) => {
            error!("Error listing movements: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error listing movements."),
            ))
        }
    }
}

pub async fn show_toner_movements(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let toner_movements: Result<Vec<Movement>, sqlx::Error> =
        sqlx::query_as(r#"SELECT * FROM movements WHERE toner_id IS NOT NULL;"#)
            .fetch_all(&state.db)
            .await;
    match toner_movements {
        Ok(movements) => {
            info!("Toner movements listed successfully");
            Ok((StatusCode::OK, Json(movements)))
        }
        Err(e) => {
            error!("Error listing toner movements: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error listing toner movements."),
            ))
        }
    }
}

pub async fn show_drum_movements(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let drum_movements: Result<Vec<Movement>, sqlx::Error> =
        sqlx::query_as(r#"SELECT * FROM movements WHERE drum_id IS NOT NULL;"#)
            .fetch_all(&state.db)
            .await;
    match drum_movements {
        Ok(movements) => {
            info!("Drum movements listed successfully");
            Ok((StatusCode::OK, Json(movements)))
        }
        Err(e) => {
            error!("Error listing drum movements: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error listing toner movements."),
            ))
        }
    }
}

pub async fn create_toner_movement(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateTonerMovementRequest>,
) -> impl IntoResponse {
    match request.toner_id {
        Some(toner_id) => {
            let printer_id: Uuid = match sqlx::query(r#"SELECT id FROM printers WHERE toner = $1;"#)
                .bind(toner_id)
                .fetch_one(&state.db)
                .await
                .unwrap()
                .try_get("id")
            {
                Ok(id) => id,
                Err(e) => {
                    error!("Error creating movement: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR;
                }
            };

            let new_movement = Movement::new(printer_id, Some(toner_id), None, request.quantity);

            // Empty quantity
            if new_movement.quantity == 0 {
                error!("Movement quantity cannot be zero.");
                return StatusCode::BAD_REQUEST;
            }

            match sqlx::query(
                r#"
                INSERT INTO movements (id, printer_id, toner_id, drum_id, quantity, created_at)
                VALUES ($1, $2, $3, $4, $5, $6);
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(printer_id)
            .bind(new_movement.toner_id)
            .bind(new_movement.drum_id)
            .bind(new_movement.quantity)
            .bind(Utc::now())
            .execute(&state.db)
            .await
            {
                Ok(_) => {
                    sqlx::query(r#"UPDATE toners SET stock = stock + $1 WHERE id = $2;"#)
                        .bind(request.quantity)
                        .bind(toner_id)
                        .execute(&state.db)
                        .await
                        .unwrap();

                    StatusCode::CREATED
                }
                Err(e) => {
                    error!("Error creating movement: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
        }
        None => {
            error!("Toner ID not found.");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn create_drum_movement(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateDrumMovementRequest>,
) -> impl IntoResponse {
    match request.drum_id {
        Some(drum_id) => {
            let printer_id: Uuid = match sqlx::query(r#"SELECT id FROM printers WHERE drum = $1;"#)
                .bind(drum_id)
                .fetch_one(&state.db)
                .await
                .unwrap()
                .try_get("id")
            {
                Ok(id) => id,
                Err(e) => {
                    error!("Error creating movement: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR;
                }
            };

            let new_movement = Movement::new(printer_id, None, Some(drum_id), request.quantity);

            // Empty quantity
            if new_movement.quantity == 0 {
                error!("Movement quantity cannot be zero.");
                return StatusCode::BAD_REQUEST;
            }

            match sqlx::query(
                r#"
                INSERT INTO movements (id, printer_id, toner_id, drum_id, quantity, created_at)
                VALUES ($1, $2, $3, $4, $5, $6);
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(printer_id)
            .bind(new_movement.toner_id)
            .bind(new_movement.drum_id)
            .bind(new_movement.quantity)
            .bind(Utc::now())
            .execute(&state.db)
            .await
            {
                Ok(_) => {
                    sqlx::query(r#"UPDATE drums SET stock = stock + $1 WHERE id = $2;"#)
                        .bind(request.quantity)
                        .bind(drum_id)
                        .execute(&state.db)
                        .await
                        .unwrap();

                    StatusCode::CREATED
                }
                Err(e) => {
                    error!("Error creating movement: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
        }
        None => {
            error!("Drum ID not found.");
            StatusCode::NOT_FOUND
        }
    }
}

pub async fn update_movement(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateMovementRequest>,
) -> impl IntoResponse {
    let movement_id = request.id;
    let new_printer_id = request.printer_id;
    let new_toner_id = request.toner_id;
    let new_drum_id = request.drum_id;
    let new_quantity = request.quantity;

    // One of the two must exist
    if new_toner_id.is_none() && new_drum_id.is_none() {
        error!("Either toner_id or drum_id must be provided.");
        return StatusCode::BAD_REQUEST;
    }

    // Not found
    match sqlx::query(r#"SELECT id FROM movements WHERE id = $1;"#)
        .bind(movement_id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            match sqlx::query(
                r#"UPDATE movements 
                    SET printer_id = $1,
                        toner_id = $2,
                        drum_id = $3,
                        quantity = $4
                    WHERE id = $5;"#,
            )
            .bind(new_printer_id)
            .bind(new_toner_id) // Optional
            .bind(new_drum_id) // Optional
            .bind(new_quantity)
            .bind(movement_id)
            .execute(&state.db)
            .await
            {
                Ok(_) => {
                    info!("Movement updated! ID: {}", &movement_id);
                    StatusCode::OK
                }
                Err(e) => {
                    error!("Error updating movement: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
        }
        Ok(None) => {
            error!("Movement ID not found.");
            StatusCode::NOT_FOUND
        }
        Err(e) => {
            error!("Error updating movement: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn delete_movement(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteRequest>,
) -> impl IntoResponse {
    match sqlx::query(r#"SELECT id FROM movements WHERE id = $1;"#)
        .bind(request.id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            match sqlx::query(r#"DELETE FROM movements WHERE id = $1;"#)
                .bind(request.id)
                .execute(&state.db)
                .await
            {
                Ok(_) => {
                    info!("Movement deleted! ID: {}", &request.id);
                    StatusCode::OK
                }
                Err(e) => {
                    error!("Error deleting movement: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
        }
        Ok(None) => {
            error!("Movement ID not found.");
            StatusCode::NOT_FOUND
        }
        Err(e) => {
            error!("Error deleting movement: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
