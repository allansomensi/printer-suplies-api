use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod brand;
pub mod database;
pub mod movement;
pub mod printer;
pub mod status;
pub mod supplies;

#[derive(Deserialize, Serialize)]
pub struct DeleteRequest {
    pub id: Uuid,
}
