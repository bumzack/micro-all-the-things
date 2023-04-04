#[macro_use]
extern crate log;

use std::io;

use warp::Filter;

mod search_movie;

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

#[tokio::main]
async fn main() -> io::Result<()> {
    pretty_env_logger::init();

    let root = warp::path::end().map(|| "Welcome to my warp server!");

    let root = root.or(search_movie::filters_search_movie::search_movie_route());

    // View access logs by setting `RUST_LOG=todos`.
    let routes = root.with(warp::log("meilisearchmovie"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 18200)).await;

    Ok(())
}
