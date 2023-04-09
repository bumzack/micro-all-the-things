use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use tokio_postgres::Row;
use warp::reject;

#[derive(Debug, Deserialize, Serialize)]
pub struct AddLogEntry {
    pub service_id: String,
    pub log_type: String,
    pub message: String,
    pub logtime: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReadLogEntry {
    pub last_n: u32,
    pub filter_log_type: Option<String>,
    pub filter_service_id: Option<String>,
    pub logtime: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogEntry {
    pub id: i32,
    pub service_id: String,
    pub log_type: String,
    pub message: String,
    pub logtime: DateTime<Utc>,
    pub created: DateTime<Utc>,
}

impl From<&Row> for LogEntry {
    fn from(value: &Row) -> Self {
        LogEntry {
            id: value.get(0),
            service_id: value.get(1),
            log_type: value.get(2),
            message: value.get(3),
            logtime: value.get(4),
            created: value.get(5),
        }
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
