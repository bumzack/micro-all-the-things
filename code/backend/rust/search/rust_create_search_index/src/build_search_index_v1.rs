use std::convert::Infallible;

use log::info;
use serde_json::json;

use common::entity::entity::{Engine, Entity};
use common::meili::meili_http::meili_http_stuff::meili_update_http;

use crate::build_search_common::{convert_to_search_index_doc, search_movies};
use crate::CLIENT;

pub async fn build_index_v1() -> Result<impl warp::Reply, Infallible> {
    let mut offset = 0;
    let limit = 100;

    let total_cnt_movies = 9_728_300;
    let mut cnt_movies = 0;

    let _msg = format!(
        "start build_index(). offset {}, limit {}, total_cnt_movies {}",
        offset, limit, total_cnt_movies
    );

    while cnt_movies < total_cnt_movies {
        let movies = search_movies(limit, offset, Engine::Meili).await;
        cnt_movies += movies.len();
        offset += limit;

        let mut docs = vec![];
        convert_to_search_index_doc(movies, &mut docs, Engine::Meili).await;

        let docs_json = json!(&docs).to_string();

        let message = format!(
            "sending a list of docs to the search index.  {} docs. movies processed {} / {}",
            docs.len(),
            cnt_movies,
            total_cnt_movies
        );
        info!("{}", &message);

        info!("starting update request for  {} docs", docs.len());
        meili_update_http(&Entity::SEARCHINDEX, &CLIENT, docs_json).await;
        info!(
            "finished update request for  {} docs.  . movies processed {} / {} ",
            docs.len(),
            cnt_movies,
            total_cnt_movies
        );
    }

    let message = format!("finished build_index(). processed {} movies ", cnt_movies);
    info!("res {}", &message);

    info!("done {}", &message);
    Ok(warp::reply::json(&message))
}
