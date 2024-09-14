use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Drum {
    pub id: Uuid,
    pub name: String,
    pub stock: i32,
}

impl Drum {
    pub fn new(name: &str) -> Self {
        Drum {
            id: Uuid::new_v4(),
            name: String::from(name),
            stock: 0,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateDrumRequest {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateDrumRequest {
    pub id: Uuid,
    pub name: String,
}
