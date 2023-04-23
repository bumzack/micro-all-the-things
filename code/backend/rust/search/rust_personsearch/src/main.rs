use std::io;
use std::time::Duration;

use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;
use warp::Filter;

mod search_person;

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

#[tokio::main]
async fn main() -> io::Result<()> {
    Builder::new().filter_level(LevelFilter::Info).init();

    let root = warp::path::end().map(|| "Welcome to my warp server!");

    let root = root.or(search_person::filters_search_person::filter_person_route());

    // View access logs by setting `RUST_LOG=todos`.
    let routes = root.with(warp::log("meilisearcperson"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 18203)).await;

    Ok(())
}
