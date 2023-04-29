use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use tokio_postgres::Row;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogInRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogOutRequest {
    pub customer_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationEntry {
    pub id: i32,
    pub customer_id: i32,
    pub jwt: Option<String>,
    pub logged_in: Option<DateTime<Utc>>,
    pub logged_out: Option<DateTime<Utc>>,
    pub created: Option<DateTime<Utc>>,
}

impl From<&Row> for AuthenticationEntry {
    fn from(value: &Row) -> Self {
        AuthenticationEntry {
            id: value.get(0),
            customer_id: value.get(1),
            jwt: value.get(2),
            logged_in: value.get(3),
            logged_out: value.get(4),
            created: value.get(5),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatedCustomer {
    pub customer_id: i32,
    pub jwt: String,
}
