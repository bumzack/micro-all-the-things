use warp::Rejection;

pub type Result<T> = std::result::Result<T, Rejection>;

pub mod db_prices {
    use deadpool_postgres::Pool;
    use log::{error, info};
    use warp::reject;

    use common::logging::logging::DivideByZero;
    use common::models::prices::{price_entry_from_row, AddPriceEntry, PriceEntry};

    use crate::db::db::TABLE_PRICE;

    pub async fn get_price(pool: Pool, tconst: &String) -> super::Result<PriceEntry> {
        let client = pool.get().await.unwrap();

        let query = format!(
            "SELECT * FROM {}  WHERE movie_tconst = '{}' ",
            TABLE_PRICE, tconst
        );

        info!("SELECT query  {}", &query);

        // TODO
        //  oh boy, that's beyond ugly
        let data = client.query_one(&query, &[]).await.map_err(|e| {
            // TODO : differentiate between errors and 404
            info!(
                "error in query one. (can be a notfoudn too, not only a hard error {:?}",
                e
            );
            // reject::custom(DivideByZero)
            reject::not_found()
        })?;

        let entry = price_entry_from_row(&data);
        Ok(entry)
    }

    pub async fn insert_price_entry(pool: Pool, entry: AddPriceEntry) -> super::Result<PriceEntry> {
        let client = pool.get().await.unwrap();
        let query = format!(
            "INSERT INTO {} (movie_tconst, amount) VALUES ($1, $2) RETURNING *",
            TABLE_PRICE
        );

        let row = client
            .query_one(query.as_str(), &[&entry.movie_tconst, &entry.amount])
            .await
            .map_err(|e| {
                error!("error rejection {:?}", e);
                reject::custom(DivideByZero)
            })?;

        let entry = price_entry_from_row(&row);
        Ok(entry)
    }
}
