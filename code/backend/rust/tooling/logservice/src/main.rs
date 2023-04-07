#[macro_use]
extern crate log;

use std::net::{SocketAddr, ToSocketAddrs};

use config::Config;
use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;

use crate::db::db::create_pool;
use crate::log_mod::log_routes::filters_logging::logging_route;

mod db;
mod log_mod;

// gotta give credit where credit is due and stuff

lazy_static::lazy_static! {
    static ref CONFIG: Config =  Config::builder()
        .add_source(config::File::with_name("/Users/bumzack/stoff/micro-all-the-things/code/backend/rust/config.toml"))
        .build()
        .unwrap();
}

// #[tokio::main(worker_threads = 2)]
#[tokio::main]
async fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();

    let pool = create_pool();

    // let cors = warp::cors()
    //     .allow_any_origin()
    //     .allow_headers(vec![
    //         "User-Agent",
    //         "Sec-Fetch-Mode",
    //         "Referer",
    //         "Origin",
    //         "Access-Control-Request-Method",
    //         "Access-Control-Request-Headers",
    //     ])
    //     .allow_methods(vec!["POST", "GET"]);

    let routes = logging_route(pool);

    let host: String = CONFIG
        .get("loggingservice_service_host")
        .expect("expected loggingservice_service_host variable");

    let port: u16 = CONFIG
        .get("loggingservice_service_port")
        .expect("expected loggingservice_service_port variable");

    let host = format!("{host}:{port}");

    info!("host {}", host);
    let socket_addrs: Vec<SocketAddr> = host.to_socket_addrs().unwrap().collect();
    let addr = socket_addrs.first().unwrap();
    warp::serve(routes).run(*addr).await;
}
