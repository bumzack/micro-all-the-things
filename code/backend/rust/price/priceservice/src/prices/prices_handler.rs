use log::{error, info};
use serde_json::json;

use common::entity::entity::Engine;
use common::models::movie::{Movie, MoviePaginationResult};
use common::models::search_doc::SearchPaginatedRequest;

use crate::{CLIENT, CONFIG};

pub mod handlers_price {
    use std::time::Instant;

    use deadpool_postgres::Pool;
    use log::{error, info};
    use reqwest::header::HeaderMap;
    use warp::{reject, Rejection, Reply};

    use common::entity::entity::Engine;
    use common::logging::tracing_headers::tracing_headers_stuff::{
        build_response_from_json, build_tracing_headers, get_trace_infos,
    };
    use common::models::prices::{AddPriceEntry, SearchPricesRequest};

    use crate::prices::db::db_prices::{get_price, get_prices, insert_price_entry};
    use crate::prices::prices_handler::search_movies;

    const SERVICE_NAME: &str = "Price Service";

    pub async fn read_price_entry(
        pool: Pool,
        tconst: String,
        headers: HeaderMap,
    ) -> Result<impl Reply, Rejection> {
        let start_total = Instant::now();

        let (initiated_by, uuid, processed_by) =
            get_trace_infos(&headers, SERVICE_NAME.to_string());

        info!(
            "reading price entry for movie title movie_tconst: {:?}",
            &tconst
        );

        let price_entry = get_price(pool, &tconst).await.map_err(|e| {
            error!("error this can be a 404 too {:?}", e);
            reject::not_found()
        })?;

        info!("found a price for tconst {}:  {:?}", &tconst, &price_entry);
        let msg = format!("found a price for tconst {}:  {:?}", &tconst, &price_entry);

        let headers = build_tracing_headers(
            &start_total,
            &SERVICE_NAME.to_string(),
            &initiated_by,
            &uuid,
            &processed_by,
            &msg,
        );

        let response = build_response_from_json(price_entry, headers);

        Ok(response)
    }

    pub async fn read_price_entries(
        req: SearchPricesRequest,
        pool: Pool,
        headers: HeaderMap,
    ) -> Result<impl Reply, Rejection> {
        let start_total = Instant::now();

        let (initiated_by, uuid, processed_by) =
            get_trace_infos(&headers, SERVICE_NAME.to_string());

        info!(
            "reading price entry for movie titles in request: {:?}",
            &req
        );

        let price_entry = get_prices(pool, &req.movie_tconst).await.map_err(|e| {
            error!("error this can be a 404 too {:?}", e);
            reject::not_found()
        })?;

        let msg = format!(
            "found {} prices for {} movie_tconsts",
            price_entry.len(),
            &req.movie_tconst.len()
        );

        let headers = build_tracing_headers(
            &start_total,
            &SERVICE_NAME.to_string(),
            &initiated_by,
            &uuid,
            &processed_by,
            &msg,
        );

        let response = build_response_from_json(price_entry, headers);

        Ok(response)
    }

    pub async fn insert_dummy_data(
        mut offset: u32,
        limit: u32,
        count: u32,
        pool: Pool,
        headers: HeaderMap,
    ) -> Result<impl Reply, Rejection> {
        let start_total = Instant::now();

        let (initiated_by, uuid, processed_by) =
            get_trace_infos(&headers, SERVICE_NAME.to_string());

        info!("inserting dummy data for all movies");

        let mut movies_found = true;
        let mut movies_processed = 0;
        let mut next_cursor_mark = Some("*".to_string());
        while movies_found && movies_processed < count {
            let (movies, next_c) =
                search_movies(limit, offset, next_cursor_mark.clone(), Engine::Solr).await;
            next_cursor_mark = next_c;
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

        let headers = build_tracing_headers(
            &start_total,
            &SERVICE_NAME.to_string(),
            &initiated_by,
            &uuid,
            &processed_by,
            &msg,
        );

        let response = build_response_from_json(msg, headers);

        Ok(response)
    }
}

async fn search_movies(
    limit: u32,
    offset: u32,
    next_cursor_mark: Option<String>,
    engine: Engine,
) -> (Vec<Movie>, Option<String>) {
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
        next_cursor_mark,
    };

    let message = format!(
        "start rust_priceservice_insert_dummy_data.search_movies().  offset {}, limit {}, sort {:?}, engine: {}",
        offset,
        limit,
        &search_request.sort.clone(),
        engine.to_string()
    );
    info!("message {}", &message);

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

    if response.is_err() {
        error!(
            "error from SearchMovie Service {:?}",
            response.err().unwrap()
        );
        return (vec![], None);
    }
    info!("XXX    search_movies all good");

    let response2 = response.unwrap();
    let paginated_result = response2
        .json::<MoviePaginationResult>()
        .await
        .expect("expected a list of Movies");
    info!(
        "rust_priceservice_insert_dummy_data.search_movies all good. found {} movies",
        paginated_result.movies.len()
    );

    let message = format!(
        "end rust_priceservice_insert_dummy_data.search_movies().  offset {}, limit {}, sort {:?}. {} movies found ,    next_cursor_mark   {:?}",
        offset,
        limit,
        &search_request.sort.clone(),
        paginated_result.movies.len(),
        paginated_result.next_cursor_mark,
    );
    info!("message {}", &message);

    info!(".rust_priceservice_insert_dummy_datasearch_movies finished successfully");

    (paginated_result.movies, paginated_result.next_cursor_mark)
}
