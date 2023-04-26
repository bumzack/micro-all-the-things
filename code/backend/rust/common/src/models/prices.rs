use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use tokio_postgres::Row;

#[derive(Debug, Deserialize, Serialize)]
pub struct AddPriceEntry {
    pub movie_tconst: String,
    pub amount: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PriceEntry {
    pub id: i32,
    pub movie_tconst: String,
    pub amount: f32,
    pub created: Option<DateTime<Utc>>,
}

pub fn price_entry_from_row(r: &Row) -> PriceEntry {
    PriceEntry {
        id: r.get(0),
        movie_tconst: r.get(1),
        amount: r.get(2),
        created: Some(r.get(3)),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchPricesRequest {
    pub movie_tconst: Vec<String>,
}
