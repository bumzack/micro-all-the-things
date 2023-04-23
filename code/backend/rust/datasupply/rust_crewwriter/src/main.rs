use std::io;
use std::time::Duration;

use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;
use warp::Filter;

use crate::crew_rest::filters_crew;

mod crew_rest;

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::builder()
            .pool_max_idle_per_host(0)
            .connection_verbose(true)
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(30))
            .no_brotli()
            .no_deflate()
            .no_gzip()
            .build()
            .unwrap();
}

#[tokio::main]
async fn main() -> io::Result<()> {
    Builder::new().filter_level(LevelFilter::Info).init();
    let root = warp::path::end().map(|| "Welcome to my warp server!");

    let root = root.or(filters_crew::crew_route());

    // View access logs by setting `RUST_LOG=todos`.
    let routes = root.with(warp::log("crewwriter"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 18102)).await;

    Ok(())
}
