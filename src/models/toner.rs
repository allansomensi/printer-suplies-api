use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Toner {
    pub id: Uuid,
    pub name: String,
    pub color: String,
}

impl Toner {
    pub fn new(name: &str, color: &str) -> Toner {
        Toner {
            id: Uuid::now_v7(),
            name: String::from(name),
            color: String::from(color),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateTonerRequest {
    pub name: String,
    pub color: String,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteTonerRequest {
    pub id: Uuid,
}
