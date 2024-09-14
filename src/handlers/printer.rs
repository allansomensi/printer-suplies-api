use std::{str::FromStr, sync::Arc};

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
    printer::{CreatePrinterRequest, Printer, UpdatePrinterRequest},
    DeleteRequest,
};

pub async fn count_printers(State(state): State<Arc<AppState>>) -> Json<i32> {
    let printer_count: Result<(i32,), sqlx::Error> =
        sqlx::query_as(r#"SELECT COUNT(*)::int FROM printers"#)
            .fetch_one(&state.db)
            .await;

    match printer_count {
        Ok((count,)) => {
            info!("Successfully retrieved printer count: {}", count);
            Json(count)
        }
        Err(e) => {
            error!("Error retrieving printer count: {e}");
            Json(0)
        }
    }
}

pub async fn search_printer(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Printer>("SELECT * FROM printers WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(printer)) => {
            info!("Printer found: {id}");
            (StatusCode::OK, Json(Some(printer)))
        }
        Ok(None) => {
            error!("No printer found.");
            (StatusCode::NOT_FOUND, Json(None))
        }
        Err(e) => {
            error!("Error retrieving printer: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
        }
    }
}

pub async fn show_printers(State(state): State<Arc<AppState>>) -> Json<Vec<Printer>> {
    match sqlx::query_as(r#"SELECT * FROM printers"#)
        .fetch_all(&state.db)
        .await
    {
        Ok(printers) => {
            info!("Printers listed successfully");
            Json(printers)
        }
        Err(e) => {
            error!("Error listing printers: {e}");
            Json(Vec::new())
        }
    }
}

pub async fn create_printer(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreatePrinterRequest>,
) -> impl IntoResponse {
    let new_printer = Printer::new(
        &request.name,
        &request.model,
        Uuid::from_str(&request.brand).unwrap(),
        Uuid::from_str(&request.toner).unwrap(),
        Uuid::from_str(&request.drum).unwrap(),
    );

    // Check duplicate
    match sqlx::query("SELECT id FROM printers WHERE name = $1")
        .bind(&new_printer.name)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            error!("Printer '{}' already exists.", &new_printer.name);
            StatusCode::CONFLICT
        }
        Ok(None) => {
            // Name is empty
            if new_printer.name.is_empty() {
                error!("Printer name cannot be empty.");
                return StatusCode::BAD_REQUEST;
            }

            // Name too short
            if new_printer.name.len() < 4 {
                error!("Printer name is too short.");
                return StatusCode::BAD_REQUEST;
            }

            // Name too long
            if new_printer.name.len() > 20 {
                error!("Printer name is too long.");
                return StatusCode::BAD_REQUEST;
            }

            match sqlx::query(
                r#"
                INSERT INTO printers (id, name, model, brand, toner, drum)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            )
            .bind(new_printer.id)
            .bind(new_printer.name)
            .bind(new_printer.model)
            .bind(new_printer.brand)
            .bind(new_printer.toner)
            .bind(new_printer.drum)
            .execute(&state.db)
            .await
            {
                Ok(_) => {
                    info!("Printer created! ID: {}", &new_printer.id);
                    StatusCode::CREATED
                }
                Err(e) => {
                    error!("Error creating printer: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn update_printer(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdatePrinterRequest>,
) -> impl IntoResponse {
    let printer_id = request.id;
    let new_name = request.name;
    let new_model = request.model;
    let new_brand = request.brand;
    let new_toner = request.toner;
    let new_drum = request.drum;

    // ID not found
    match sqlx::query(r#"SELECT id FROM printers WHERE id = $1"#)
        .bind(printer_id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            // Name is empty
            if new_name.is_empty() {
                error!("Printer name cannot be empty.");
                return StatusCode::BAD_REQUEST;
            }

            // Name too short
            if new_name.len() < 4 {
                error!("Printer name is too short.");
                return StatusCode::BAD_REQUEST;
            }

            // Name too long
            if new_name.len() > 20 {
                error!("Printer name is too long.");
                return StatusCode::BAD_REQUEST;
            }

            // Check duplicate
            match sqlx::query(r#"SELECT id FROM printers WHERE name = $1 AND id != $2"#)
                .bind(&new_name)
                .bind(printer_id)
                .fetch_optional(&state.db)
                .await
            {
                Ok(Some(_)) => {
                    error!("Printer name already exists.");
                    return StatusCode::BAD_REQUEST;
                }
                Ok(None) => {
                    match sqlx::query(
                        r#"UPDATE printers 
                    SET name = $1, model = $2, brand = $3, toner = $4, drum = $5 
                    WHERE id = $6"#,
                    )
                    .bind(&new_name)
                    .bind(&new_model)
                    .bind(&new_brand)
                    .bind(&new_toner)
                    .bind(&new_drum)
                    .bind(printer_id)
                    .execute(&state.db)
                    .await
                    {
                        Ok(_) => {
                            info!("Printer updated! ID: {}", &printer_id);
                            StatusCode::OK
                        }
                        Err(e) => {
                            error!("Error updating printer: {}", e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        }
                    }
                }
                Err(e) => {
                    error!("Error checking for duplicate printer name: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
        }
        Ok(None) => {
            error!("Printer ID not found.");
            StatusCode::NOT_FOUND
        }
        Err(e) => {
            error!("Error fetching printer by ID: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn delete_printer(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteRequest>,
) -> impl IntoResponse {
    match sqlx::query(r#"SELECT id FROM printers WHERE id = $1"#)
        .bind(request.id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            match sqlx::query(r#"DELETE FROM printers WHERE id = $1"#)
                .bind(request.id)
                .execute(&state.db)
                .await
            {
                Ok(_) => {
                    info!("Printer deleted! ID: {}", &request.id);
                    StatusCode::OK
                }
                Err(e) => {
                    error!("Error deleting printer: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
        }
        Ok(None) => {
            error!("Printer ID not found.");
            StatusCode::NOT_FOUND
        }
        Err(e) => {
            error!("Error deleting printer: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
