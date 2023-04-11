use warp::Rejection;

pub type Result<T> = std::result::Result<T, Rejection>;

pub mod db_logging {
    use deadpool_postgres::Pool;
    use log::{error, info};
    use warp::reject;

    use common::logging::logging::{AddLogEntry, DivideByZero, LogEntry, ReadLogEntry};

    use crate::db::db::TABLE_LOG_ENTRY;

    pub async fn list_entries(pool: Pool, req: ReadLogEntry) -> super::Result<Vec<LogEntry>> {
        let client = pool.get().await.unwrap();

        let query_items = format!("SELECT * FROM {}   ", TABLE_LOG_ENTRY);
        let order_by = " ORDER BY created DESC ".to_string();
        let limit = format!(" LIMIT  {} ", req.last_n);

        let filter_log_type = match req.filter_log_type {
            Some(t) => format!(" log_type  = \"{}\" ", t),
            None => "".to_string(),
        };

        let service_id = match req.filter_service_id {
            Some(t) => format!(" service_id  = \"{}\" ", t),
            None => "".to_string(),
        };

        let mut filters = vec![filter_log_type, service_id];
        let filters: Vec<String> = filters.drain(..).filter(|s| !s.is_empty()).collect();

        let filter = if !filters.is_empty() {
            let filters = filters.join(" AND ");
            format!(" WHERE {filters}")
        } else {
            "".to_string()
        };

        let query = format!("{query_items} {filter} {order_by} {limit}");

        info!("SELECT query  {}", &query);

        let data = client.query(&query, &[]).await.unwrap();
        let log_entries = data.iter().map(|r| LogEntry::from(r)).collect();

        Ok(log_entries)
    }

    pub async fn insert_log_entry(pool: Pool, req: AddLogEntry) -> super::Result<LogEntry> {
        let client = pool.get().await.unwrap();
        let query = format!(
            "INSERT INTO {} (service_id, log_type, message, logtime) VALUES ($1, $2, $3, $4) RETURNING *",
            TABLE_LOG_ENTRY
        );

        let row = client
            .query_one(
                query.as_str(),
                &[&req.service_id, &req.log_type, &req.message, &req.logtime],
            )
            .await
            .map_err(|e| {
                error!("error rejection {:?}", e);
                reject::custom(DivideByZero)
            })?;
        let entry = LogEntry::from(&row);
        Ok(entry)
    }
}
