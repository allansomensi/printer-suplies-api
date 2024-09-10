use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "toner_color", rename_all = "lowercase")]
#[serde(rename_all = "snake_case")]
pub enum TonerColor {
    Black,
    Cyan,
    Yellow,
    Magenta,
}

impl FromStr for TonerColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "black" => Ok(TonerColor::Black),
            "cyan" => Ok(TonerColor::Cyan),
            "yellow" => Ok(TonerColor::Yellow),
            "magenta" => Ok(TonerColor::Magenta),
            _ => Err(format!("Cor inválida! Erro: {}", s)),
        }
    }
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct Toner {
    pub id: Uuid,
    pub name: String,
    pub color: TonerColor,
}

impl Toner {
    pub fn new(name: &str, color: TonerColor) -> Self {
        Toner {
            id: Uuid::new_v4(),
            name: String::from(name),
            color,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateTonerRequest {
    pub name: String,
    pub color: TonerColor,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateTonerRequest {
    pub id: Uuid,
    pub name: String,
    pub color: TonerColor,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteTonerRequest {
    pub id: Uuid,
}
