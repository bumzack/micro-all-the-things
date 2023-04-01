use std::io;
use warp::Filter;

mod create_search_index;

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let root = warp::path::end().map(|| "Welcome to my warp server!");

    let root = root.or(create_search_index::filters_search_movie::create_search_index_route());

    // View access logs by setting `RUST_LOG=todos`.
    let routes = root.with(warp::log("create_search_index"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 18200)).await;

    Ok(())
}
