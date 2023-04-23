#[macro_use]
extern crate log;

use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;

use config::Config;
use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;

use crate::customerprices::customerprices_routes::filters_logging::price_route;
use crate::db::db::create_pool;

mod customerprices;
mod db;

// gotta give credit where credit is due and stuff

lazy_static::lazy_static! {
    static ref CONFIG: Config =  Config::builder()
        .add_source(config::File::with_name("/Users/bumzack/stoff/micro-all-the-things/code/backend/rust/config.toml"))
        .build()
        .unwrap();
}

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::builder()
           //  .pool_max_idle_per_host(0)
//             .connection_verbose(true)
            .timeout(Duration::from_secs(300))
            .connect_timeout(Duration::from_secs(300))
            .no_brotli()
            .no_deflate()
            .no_gzip()
            .build()
            .unwrap();
}

// #[tokio::main(worker_threads = 2)]
#[tokio::main]
async fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();

    let pool = create_pool();

    let routes = price_route(pool);

    let host: String = CONFIG
        .get("customerpriceservice_service_host")
        .expect("expected customerpriceservice_service_host variable");

    let port: u16 = CONFIG
        .get("customerpriceservice_service_port")
        .expect("expected customerpriceservice_service_port variable");

    let host = format!("{host}:{port}");

    info!("customerprice service host {}", host);
    let socket_addrs: Vec<SocketAddr> = host.to_socket_addrs().unwrap().collect();
    let addr = socket_addrs.first().unwrap();
    warp::serve(routes).run(*addr).await;
}
