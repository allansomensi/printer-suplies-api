use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

type Version = String;

#[derive(Deserialize, Serialize)]
pub struct Database {
    pub version: Version,
    pub max_connections: u16,
    pub opened_connections: u16,
}

#[derive(Deserialize, Serialize)]
pub struct Dependencies {
    pub database: Database,
}

#[derive(Deserialize, Serialize)]
pub struct Status {
    pub updated_at: DateTime<Utc>,
    pub dependencies: Dependencies,
}
