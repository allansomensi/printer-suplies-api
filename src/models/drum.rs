use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Drum {
    pub id: Uuid,
    pub name: String,
}

impl Drum {
    pub fn new(name: &str) -> Drum {
        Drum {
            id: Uuid::now_v7(),
            name: String::from(name),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateDrumRequest {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteDrumRequest {
    pub id: Uuid,
}
