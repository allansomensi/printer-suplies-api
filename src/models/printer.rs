use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Printer {
    pub id: Uuid,
    pub name: String,
    pub model: String,
    pub brand: Uuid,
    pub toner: Uuid,
    pub drum: Uuid,
}

impl Printer {
    pub fn new(name: &str, model: &str, brand: Uuid, toner: Uuid, drum: Uuid) -> Self {
        Printer {
            id: Uuid::new_v4(),
            name: String::from(name),
            model: String::from(model),
            brand,
            toner,
            drum,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreatePrinterRequest {
    pub name: String,
    pub model: String,
    pub brand: String,
    pub toner: String,
    pub drum: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdatePrinterRequest {
    pub id: Uuid,
    pub name: String,
    pub model: String,
    pub brand: String,
    pub toner: String,
    pub drum: String,
}
