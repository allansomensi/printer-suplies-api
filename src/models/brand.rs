use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Brand {
    pub id: Uuid,
    pub name: String,
}

impl Brand {
    pub fn new(name: &str) -> Self {
        Brand {
            id: Uuid::now_v7(),
            name: String::from(name),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateBrandRequest {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteBrandRequest {
    pub id: Uuid,
}
