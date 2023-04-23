use std::io;
use std::time::Duration;

use config::Config;
use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;
use warp::Filter;

mod build_search_common;
mod build_search_index;
mod build_search_index_v1;
mod build_search_index_v2;
mod build_search_index_v3;
mod pagination_manager;

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

lazy_static::lazy_static! {
    static ref CONFIG: Config = Config::builder()
        .add_source(config::File::with_name("/Users/bumzack/stoff/micro-all-the-things/code/backend/rust/config.toml"))
        .build()
        .unwrap();
}

#[tokio::main]
async fn main() -> io::Result<()> {
    Builder::new().filter_level(LevelFilter::Info).init();

    let root = warp::path::end().map(|| "Welcome to my warp server!");

    let root = root.or(build_search_index::filters_build_index::build_index_route());

    // View access logs by setting `RUST_LOG=todos`.
    let routes = root.with(warp::log("build_search_index"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 18300)).await;

    Ok(())
}
