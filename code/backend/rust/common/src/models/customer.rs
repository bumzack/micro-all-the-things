use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use tokio_postgres::Row;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddCustomer {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created: Option<DateTime<Utc>>,
}

impl From<&Row> for Customer {
    fn from(value: &Row) -> Self {
        Customer {
            id: value.get(0),
            first_name: value.get(1),
            last_name: value.get(2),
            email: value.get(3),
            password: value.get(4),
            created: value.get(5),
        }
    }
}
