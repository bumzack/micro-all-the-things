pub mod filters_logging {
    use common::logging::{AddLogEntry, DivideByZero, ReadLogEntry};
    use deadpool_postgres::Pool;
    use warp::{reject, Rejection, Reply};
    use warp::reply::json;

    use crate::log_mod::db::db_logging::{insert_log_entry, list_entries};
    use crate::prices::db::db_logging::{get_pric, get_price};

    pub async fn insert_log_entry_handler(
        pool: Pool,
        req: AddLogEntry,
    ) -> Result<impl Reply, Rejection> {
        info!("adding prices entry {:?}", req);

        let x = &insert_log_entry(pool.clone(), req)
            .await
            // TODO fix CustomError
            .map_err(|e| {
                error!("error rejection {:?}", e);
                reject::custom(DivideByZero)
            })?;

        Ok(json(&x))
    }

    pub async fn read_price_entry(pool: Pool, tconst: String) -> Result<impl Reply, Rejection> {
        info!("reading prices entries {:?}", &tconst);

        let data = get_price(pool, tconst)
            .await
            // TODO fix CustomError
            .map_err(|e| {
                error!("error rejection {:?}", e);
                reject::custom(DivideByZero)
            })?;

        Ok(json(&data))
    }
}
