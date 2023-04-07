pub mod filters_logging {
    use deadpool_postgres::Pool;
    use log::{error, info};
    use warp::{reject, Rejection, Reply};
    use warp::reply::json;

    use common::logging::{AddLogEntry, DivideByZero, ReadLogEntry};

    use crate::log_mod::db::db_logging::{insert_log_entry, list_entries};

    pub async fn insert_log_entry_handler(
        pool: Pool,
        req: AddLogEntry,
    ) -> Result<impl Reply, Rejection> {
        info!("adding log_mod entry {:?}", req);

        let x = &insert_log_entry(pool.clone(), req)
            .await
            // TODO fix CustomError
            .map_err(|e| {
                error!("error rejection {:?}", e);
                reject::custom(DivideByZero)
            })?;

        Ok(json(&x))
    }

    pub async fn read_log_entries(pool: Pool, req: ReadLogEntry) -> Result<impl Reply, Rejection> {
        info!("reading log_mod entries {:?}", &req);

        let data = list_entries(pool, req)
            .await
            // TODO fix CustomError
            .map_err(|e| {
                error!("error rejection {:?}", e);
                reject::custom(DivideByZero)
            })?;

        Ok(json(&data))
    }
}
