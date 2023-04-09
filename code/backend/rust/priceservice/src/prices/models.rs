use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use tokio_postgres::Row;
use warp::reject;

#[derive(Debug, Deserialize, Serialize)]
pub struct AddPriceEntry {
    pub movie_tconst: String,
    pub amount: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PriceEntry {
    pub id: u32,
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

impl reject::Reject for DivideByZero {}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("error executing DB query: {0}")]
    DBQueryError(#[from] tokio_postgres::Error),
    // #[error("error creating table: {0}")]
    // DBInitError(tokio_postgres::Error),
    // #[error("error reading file: {0}")]
    // ReadFileError(#[from] std::io::Error),
}

impl reject::Reject for MyError {}

// TODO: hihihii
#[derive(Debug)]
pub struct DivideByZero;
