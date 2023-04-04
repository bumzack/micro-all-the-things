use std::convert::Infallible;

use serde_json::json;

use common::entity::handlers_entity::exec_meilisearch_update;
use common::logging_service_client::logging_service;

use crate::build_search_common::{convert_to_meilisearch_doc, search_movies};
use crate::CLIENT;

pub async fn build_index_v1() -> Result<impl warp::Reply, Infallible> {
    let mut offset = 0;
    let limit = 100;

    let total_cnt_movies = 9_728_300;
    let mut cnt_movies = 0;

    let msg = format!(
        "start build_index(). offset {}, limit {}, total_cnt_movies {}",
        offset, limit, total_cnt_movies
    );
    logging_service::log_entry(
        "rust_create_search_index".to_string(),
        "INFO".to_string(),
        &msg,
    )
    .await;

    while cnt_movies < total_cnt_movies {
        let movies = search_movies(limit, offset).await;
        cnt_movies += movies.len();
        offset += limit;

        let mut docs = vec![];
        convert_to_meilisearch_doc(movies, &mut docs).await;

        let docs_json = json!(&docs).to_string();

        let message = format!(
            "sending a list of docs to the search index.  {} docs. movies processed {} / {}",
            docs.len(),
            cnt_movies,
            total_cnt_movies
        );
        info!("{}", &message);

        logging_service::log_entry(
            "rust_create_search_index".to_string(),
            "INFO".to_string(),
            &message,
        )
        .await;

        info!("starting update request for  {} docs", docs.len());
        exec_meilisearch_update(&"searchindex".to_string(), &CLIENT, docs_json).await;
        info!(
            "finished update request for  {} docs.  . movies processed {} / {} ",
            docs.len(),
            cnt_movies,
            total_cnt_movies
        );
    }

    let message = format!("finished build_index(). processed {} movies ", cnt_movies);
    info!("res {}", &message);
    logging_service::log_entry(
        "rust_create_search_index".to_string(),
        "INFO".to_string(),
        &message,
    )
    .await;
    info!("done {}", &message);
    Ok(warp::reply::json(&message))
}
