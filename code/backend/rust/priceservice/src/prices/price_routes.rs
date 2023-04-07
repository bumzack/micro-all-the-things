pub mod filters_logging {
    use deadpool_postgres::Pool;
    use warp::{Filter, Rejection, Reply};

    use crate::db::server::with_db;
    use crate::prices::prices_handler::filters_logging::read_price_entry;

    pub fn price_route(
        pool: Pool,
    ) -> impl Filter<Extract=(impl Reply, ), Error=Rejection> + Clone {
        let server1 = warp::path!("api" / "price" / String);
        let search_name = server1
            .and(with_db(pool.clone()))
            .and(warp::get())
            .and_then(|pool: Pool, tconst| {
                info!("GET /api/price/:tconst");
                read_price_entry(pool, tconst)
            });

        let server3 = warp::path!("api" / "log" / "entry");
        let search_nconsts = server3
            .and(with_db(pool.clone()))
            .and(warp::post())
            .and(json_body_add_log_entry())
            .and_then(|pool: Pool, req: AddLogEntry| {
                info!("POST  /api/prices/entry  matched");
                log_entry(&req);
                insert_log_entry_handler(pool, req)
            });

        search_name.or(search_nconsts)
        //     .or(search_nconst)
    }

    fn log_entry(req: &AddLogEntry) {
        match req.log_type.to_ascii_lowercase().as_str() {
            "info" => {
                info!(
                    "service_id {}, logtime {:?}: {}",
                    &req.service_id, &req.logtime, &req.message
                );
            }
            "error" => {
                error!(
                    "service_id {}, logtime {:?}: {}",
                    &req.service_id, &req.logtime, &req.message
                );
            }
            "debug" => {
                info!(
                    "service_id {}, logtime {:?}: {}",
                    &req.service_id, &req.logtime, &req.message
                );
            }
            _ => {
                info!(
                    "unknowne log_type {}.  service_id {}, logtime {}: {}",
                    &req.log_type, &req.service_id, &req.logtime, &req.message
                );
            }
        }
        info!("log entry {:?}", &req);
    }

    fn json_body_read_log_entry() -> impl Filter<Extract=(AddPriceEntry, ), Error=warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 1000 * 1000).and(warp::body::json())
    }
}
