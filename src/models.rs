use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Asset {
    pub id: i64,
    pub name: String,
    pub unit_value: f64,
    pub quantity: f64,
}

#[derive(Serialize)]
pub struct PortfolioSummary {
    pub total_value: f64,
    pub total_assets: usize,
}

pub struct UserRecord {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}
