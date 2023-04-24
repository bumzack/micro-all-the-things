use std::io;

use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;
use warp::Filter;

mod search_movieaka;

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

    let root = root.or(search_movieaka::filters_search_movieaka::filter_movieaka_route());

    // View access logs by setting `RUST_LOG=todos`.
    let routes = root.with(warp::log("meilisearchmovieaka"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 18201)).await;

    Ok(())
}
