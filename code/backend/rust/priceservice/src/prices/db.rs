use warp::Rejection;

pub type Result<T> = std::result::Result<T, Rejection>;

pub mod db_logging {
    use common::logging::{AddLogEntry, LogEntry};
    use common::logging::MyError::DBQueryError;
    use deadpool_postgres::Pool;
    use log::Level::Error;

    use crate::db::db::TABLE_PRICE;
    use crate::prices::models::{AddPriceEntry, price_entry_from_row, PriceEntry};

    pub async fn get_price(pool: Pool, tconst: String) -> super::Result<Option<PriceEntry>> {
        let client = pool.get().await.unwrap();

        let query = format!("SELECT * FROM {}  WHERE tconst = {} ", TABLE_PRICE, tconst);

        info!("SELECT query  {}", &query);

        // TODO
        //  oh boy, that's beyond ugly
        let mut data = client.query(&query, &[]).await.unwrap();
        if data.is_empty() {
            return Ok(None);
        }
        let mut log_entries: Vec<PriceEntry> =
            data.drain(0..1).map(|r| price_entry_from_row(&r)).collect();
        let entry = log_entries.remove(0);
        Ok(Some(entry))
    }

    pub async fn insert_price_entry(pool: Pool, entry: AddPriceEntry) -> super::Result<LogEntry> {
        let client = pool.get().await.unwrap();
        let query = format!(
            "INSERT INTO {} (movie_tconst, amount, message, logtime) VALUES ($1, $2, $3, $4) RETURNING *",
            TABLE_PRICE
        );

        let row = client
            .query_one(
                query.as_str(),
                &[&req.service_id, &req.log_type, &req.message, &req.logtime],
            )
            .await
            .map_err(DBQueryError)?;
        let entry = LogEntry::from(&row);
        Ok(entry)
    }
}
