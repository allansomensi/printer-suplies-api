use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Toner {
    pub id: Uuid,
    pub name: String,
    pub stock: i32,
}

impl Toner {
    pub fn new(name: &str) -> Self {
        Toner {
            id: Uuid::new_v4(),
            name: String::from(name),
            stock: 0,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateTonerRequest {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateTonerRequest {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteTonerRequest {
    pub id: Uuid,
}
