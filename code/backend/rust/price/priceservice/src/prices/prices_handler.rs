use serde_json::json;

use common::entity::entity::Engine;
use common::logging::logging_service_client::logging_service;
use common::logging::logging_service_client::logging_service::log_external_service_error;
use common::models::movie::Movie;
use common::models::search_doc::SearchPaginatedRequest;

use crate::{CLIENT, CONFIG};

pub mod handlers_price {
    use deadpool_postgres::Pool;
    use warp::http::StatusCode;
    use warp::reply::json;
    use warp::{reject, Rejection, Reply};

    use common::entity::entity::Engine;
    use common::models::prices::AddPriceEntry;

    use crate::prices::db::db_logging::{get_price, insert_price_entry};
    use crate::prices::prices_handler::search_movies;

    pub async fn read_price_entry(pool: Pool, tconst: String) -> Result<impl Reply, Rejection> {
        info!(
            "reading price entry for movie title movie_tconst: {:?}",
            &tconst
        );

        let price_entry = get_price(pool, tconst).await.map_err(|e| {
            error!("error this can be a 404 too {:?}", e);
            reject::not_found()
        })?;

        Ok(json(&price_entry))
    }

    pub async fn insert_dummy_data(
        mut offset: u32,
        limit: u32,
        count: u32,
        pool: Pool,
    ) -> Result<impl Reply, Rejection> {
        info!("inserting dummy data for all movies");

        let mut movies_found = true;
        let mut movies_processed = 0;
        while movies_found && movies_processed < count {
            let movies = search_movies(limit, offset, Engine::Solr).await;
            movies_found = !movies.is_empty();
            for m in movies {
                let amount = 15.0 + rand::random::<f32>() * 15.0;
                let amount = (amount * 100.0).round() / 100.0;

                let add_price_entry = AddPriceEntry {
                    movie_tconst: m.tconst.clone(),
                    amount,
                };
                let _ = insert_price_entry(pool.clone(), add_price_entry).await;
                movies_processed += 1;
            }
            offset += limit;
        }
        let msg = format!("movies processed {}", movies_processed);
        Ok(warp::reply::with_status(msg, StatusCode::CREATED))
    }
}

async fn search_movies(limit: u32, offset: u32, engine: Engine) -> Vec<Movie> {
    info!("rust_priceservice_insert_dummy_data.search_movies");
    let search_movie: String = CONFIG
        .get("search_movie")
        .expect("expected search_movie URL");

    let search_movie = search_movie.replace("ENGINE", &engine.to_string());

    let search_request = SearchPaginatedRequest {
        q: "*".to_string(),
        offset,
        limit,
        sort: vec!["tconst:asc".to_string()],
    };

    let message = format!(
        "start rust_priceservice_insert_dummy_data.search_movies().  offset {}, limit {}, sort {:?}, engine: {}",
        offset,
        limit,
        &search_request.sort.clone(),
        engine.to_string()
    );
    info!("message {}", &message);
    logging_service::log_entry(
        " rust_priceservice_insert_dummy_data".to_string(),
        "INFO".to_string(),
        &message,
    )
    .await;

    info!("search movie URL {}", &search_movie);
    let json = json!(&search_request);
    let response = CLIENT.post(search_movie).json(&json).send().await;

    let message = format!(
        "error rust_priceservice_insert_dummy_data.search_movies(). offset {}, limit {}, sort {:?}.",
        offset,
        limit,
        &search_request.sort.clone()
    );
    let msg = "search for movies paginated search request".to_string();
    log_external_service_error(&msg, &message, &response).await;

    if response.is_err() {
        error!(
            "error from SearchMovie Service {:?}",
            response.err().unwrap()
        );
        return vec![];
    }
    info!("XXX    search_movies all good");

    let response2 = response.unwrap();
    let movies = response2
        .json::<Vec<Movie>>()
        .await
        .expect("expected a list of Movies");
    info!(
        "rust_priceservice_insert_dummy_data.search_movies all good. found {} movies",
        movies.len()
    );

    let message = format!(
        "end rust_priceservice_insert_dummy_data.search_movies().  offset {}, limit {}, sort {:?}. {} movies found ",
        offset,
        limit,
        &search_request.sort.clone(),
        movies.len()
    );
    info!("message {}", &message);
    logging_service::log_entry(
        " rust_priceservice_insert_dummy_data".to_string(),
        "INFO".to_string(),
        &message,
    )
    .await;
    info!(".rust_priceservice_insert_dummy_datasearch_movies finished successfully");

    movies
}
