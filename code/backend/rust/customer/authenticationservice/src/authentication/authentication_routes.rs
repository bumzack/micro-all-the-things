pub mod handler_authentication {
    use common::models::authentication::{LogInRequest, LogOutRequest};
    use deadpool_postgres::Pool;
    use warp::{Filter, Rejection, Reply};

    use crate::authentication::authentication_handler::handler_authentication::{
        check_login_handler, login_handler, logout_handler,
    };
    use crate::db::server::with_db;

    pub fn authentication_route(
        pool: Pool,
    ) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
        let server1 = warp::path!("api" / "v1" / "authentication" / i32);
        let authentication = server1
            .and(with_db(pool.clone()))
            .and(warp::get())
            .and_then(|customer_id: i32, pool: Pool| {
                info!("GET /api/v1/authentication/:customer_id");
                check_login_handler(pool, customer_id)
            });

        let server1 = warp::path!("api" / "v1" / "authentication" / "login");
        let authentication_login = server1
            .and(with_db(pool.clone()))
            .and(warp::post())
            .and(json_body_login())
            .and_then(|pool: Pool, login_request| {
                info!("POST /api/v1/authentication/login");
                login_handler(pool, login_request)
            });

        let server1 = warp::path!("api" / "v1" / "authentication" / "logout");
        let authentication_logout = server1
            .and(with_db(pool.clone()))
            .and(warp::post())
            .and(json_body_logout())
            .and_then(|pool: Pool, logout_request: LogOutRequest| {
                info!("POST /api/v1/authentication/logout");
                logout_handler(pool, logout_request)
            });

        // let server1 =
        //     warp::path!("api" / "v1" / "authentication" / "insertdummydata" / u32 / u32 / u32);
        // let insert_dummy_data = server1
        //     .and(with_db(pool.clone()))
        //     .and(warp::get())
        //     .and_then(|offset: u32, limit: u32, count: u32, pool: Pool| {
        //         info!("GET /api/v1/authentication/insertdummydata");
        //         insert_dummy_data_handler(offset, limit, count, pool)
        //     });

        authentication
            .or(authentication_login)
            .or(authentication_logout)
    }

    fn json_body_login() -> impl Filter<Extract = (LogInRequest,), Error = warp::Rejection> + Clone
    {
        warp::body::content_length_limit(1024 * 1000 * 1000).and(warp::body::json())
    }

    fn json_body_logout() -> impl Filter<Extract = (LogOutRequest,), Error = warp::Rejection> + Clone
    {
        warp::body::content_length_limit(1024 * 1000 * 1000).and(warp::body::json())
    }
}
