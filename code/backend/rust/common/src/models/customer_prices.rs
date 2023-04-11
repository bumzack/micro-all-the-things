use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use tokio_postgres::Row;

#[derive(Debug, Deserialize, Serialize)]
pub struct AddCustomerPriceEntry {
    pub customer_id: i32,
    pub start_year: i32,
    pub end_year: i32,
    pub discount: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomerPriceEntry {
    pub id: i32,
    pub customer_id: i32,
    pub discount: f32,
    pub start_year: i32,
    pub end_year: i32,
    pub created: Option<DateTime<Utc>>,
}

impl From<&Row> for CustomerPriceEntry {
    fn from(value: &Row) -> Self {
        CustomerPriceEntry {
            id: value.get(0),
            customer_id: value.get(1),
            discount: value.get(2),
            start_year: value.get(3),
            end_year: value.get(4),
            created: Some(value.get(5)),
        }
    }
}
