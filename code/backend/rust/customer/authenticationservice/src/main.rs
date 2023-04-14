#[macro_use]
extern crate log;

use std::net::{SocketAddr, ToSocketAddrs};

use config::Config;
use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;

use crate::authentication::authentication_routes::handler_authentication::authentication_route;
use crate::db::db::create_pool;

mod authentication;
mod db;

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

// gotta give credit where credit is due and stuff

lazy_static::lazy_static! {
    static ref CONFIG: Config =  Config::builder()
        .add_source(config::File::with_name("/home/bumzack/micro-all-the-things/code/backend/rust/config.toml"))
        .build()
        .unwrap();
}

#[tokio::main]
async fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();

    let pool = create_pool();

    let routes = authentication_route(pool);

    let host: String = CONFIG
        .get("authenticationservice_service_host")
        .expect("expected authenticationservice_service_host variable");

    let port: u16 = CONFIG
        .get("authenticationservice_service_port")
        .expect("expected authenticationservice_service_port  variable");

    let host = format!("{host}:{port}");

    info!("authenticationservice  host {}", host);
    let socket_addrs: Vec<SocketAddr> = host.to_socket_addrs().unwrap().collect();
    let addr = socket_addrs.first().unwrap();
    warp::serve(routes).run(*addr).await;
}
