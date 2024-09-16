use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Movement {
    pub id: Uuid,
    pub printer_id: Uuid,
    pub toner_id: Option<Uuid>,
    pub drum_id: Option<Uuid>,
    pub quantity: i32,
    pub created_at: DateTime<Utc>,
}

impl Movement {
    pub fn new(
        printer_id: Uuid,
        toner_id: Option<Uuid>,
        drum_id: Option<Uuid>,
        quantity: i32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            printer_id,
            toner_id,
            drum_id,
            quantity,
            created_at: Utc::now(),
        }
    }
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct CreateTonerMovementRequest {
    pub toner_id: Option<Uuid>,
    pub quantity: i32,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct CreateDrumMovementRequest {
    pub drum_id: Option<Uuid>,
    pub quantity: i32,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct UpdateMovementRequest {
    pub id: Uuid,
    pub printer_id: Uuid,
    pub toner_id: Option<Uuid>,
    pub drum_id: Option<Uuid>,
    pub quantity: i32,
}
