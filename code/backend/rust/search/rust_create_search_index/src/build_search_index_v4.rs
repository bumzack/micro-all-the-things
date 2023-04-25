use std::collections::{HashMap, HashSet};
use std::convert::Infallible;

use log::{error, info};
use serde_json::json;
use tokio::time::Instant;
use warp::http::HeaderMap;

use common::entity::entity::{Engine, Entity};
use common::helper::dump_response_status;
use common::logging::logging_service_client::logging_service;
use common::meili::meili_http::meili_http_stuff::meili_update_http;
use common::models::movie::{Movie, MoviePaginationResult};
use common::models::person::{Person, SearchPersonList, SearchPrincipalList};
use common::models::principal::Principal;
use common::models::search_doc::{SearchIndexDoc, SearchPaginatedRequest};
use common::solr::solr_http::mod_solr_http::solr_update_http;

use crate::{CLIENT, CONFIG};
use crate::build_search_common::prepare_for_request;

pub async fn build_index_v4(
    engine: Engine,
    offset: u32,
    limit: u32,
    max_movies: u32,
    _tasks: u32,
    _headers: HeaderMap,
) -> Result<impl warp::Reply, Infallible> {
    start_tasks_v4(
        max_movies as usize,
        offset as usize,
        limit as usize,
        engine.clone(),
    )
        .await;

    let message = "processed stuff ".to_string();
    info!("done {}", &message);
    Ok(warp::reply::json(&message))
}

async fn start_tasks_v4(max_movies: usize, offset: usize, limit: usize, engine: Engine) {
    let mut movies_processed = 0;
    let mut next_cursor_mark = Some("*".to_string());

    let mut offset = offset;

    while movies_processed < max_movies {
        info!(
            "limit {}, offset {},  movies_processed   {}, max_movies   {} ",
            limit, offset, movies_processed, max_movies
        );

        let (cnt_movies, n) =
            search_and_write_to_index_v4(offset, limit, next_cursor_mark, engine.clone()).await;
        next_cursor_mark = n;
        offset += cnt_movies;
        movies_processed += cnt_movies;

        info!(
            "limit {}, offset {},  movies_processed   {}, max_movies   {}   next_cursor_mark {:?}",
            limit, offset, movies_processed, max_movies, next_cursor_mark,
        );

        if cnt_movies == 0 {
            info!(
                "limit {}, offset {},  movies_processed   {}, max_movies   {}   next_cursor_mark {:?}, no new moves found -> quitting",
                limit, offset, movies_processed, max_movies, next_cursor_mark,
            );
            return;
        }
    }
}

async fn search_and_write_to_index_v4(
    offset: usize,
    limit: usize,
    next_cursor_mark: Option<String>,
    engine: Engine,
) -> (usize, Option<String>) {
    let start = Instant::now();

    let paginated_movie_result =
        search_movies_v4(limit, offset, next_cursor_mark, engine.clone()).await;

    let movies = paginated_movie_result.movies;
    let next_cursor_mark = paginated_movie_result.next_cursor_mark;
    if movies.is_empty() {
        return (0, None);
    }
    let cnt = movies.len();
    let docs: Vec<SearchIndexDoc> = convert_to_search_index_doc_v4(movies, engine.clone()).await;

    let docs_json = json!(&docs).to_string();

    info!(
        "starting update request for  {} docs. offset {}, limit {}",
        docs.len(),
        offset,
        limit
    );
    let entity = Entity::SEARCHINDEX;
    meili_update_http(&entity, &CLIENT, docs_json.clone()).await;
    solr_update_http(&entity, &CLIENT, docs_json).await;

    let elapsed = start.elapsed();
    info!(
        "search_and_write_to_index_v4_request. processed {} docs. offset {}, limit {}.  duration {} ms",
        docs.len(),
        offset,
        limit,
        elapsed.as_millis(),
    );

    (cnt, next_cursor_mark)
}

pub async fn search_movies_v4(
    limit: usize,
    offset: usize,
    next_cursor_mark: Option<String>,
    engine: Engine,
) -> MoviePaginationResult {
    let search_movie: String = CONFIG
        .get("search_movie")
        .expect("expected search_movie URL");

    let search_movie = search_movie.replace("ENGINE", &engine.to_string());

    let search_request = SearchPaginatedRequest {
        q: "*".to_string(),
        offset: offset as u32,
        limit: limit as u32,
        sort: vec!["tconst:asc".to_string()],
        next_cursor_mark: next_cursor_mark.clone(),
    };

    let message = format!(
        "start search_movies().  offset {}, limit {}, sort {:?}, engine: {},   next_cursor_mark {:?}",
        offset,
        limit,
        &search_request.sort.clone(),
        engine.to_string(),
        next_cursor_mark,
    );
    info!("{}", &message);

    let json = json!(&search_request);
    let response = CLIENT.post(search_movie).json(&json).send().await;

    if response.is_err() {
        error!(
            "error from SearchMovie Service {:?}",
            response.err().unwrap()
        );
        let res = MoviePaginationResult {
            movies: vec![],
            next_cursor_mark: None,
        };
        return res;
    }

    let response2 = response.unwrap();
    let movies_paginated_result = response2
        .json::<MoviePaginationResult>()
        .await
        .expect("expected a list of Movies");

    let message = format!(
        "XXXx end search_movies().  offset {}, limit {}, sort {:?}. {} movies found. next_cursor_mark {:?} ",
        offset,
        limit,
        &search_request.sort.clone(),
        movies_paginated_result.movies.len(),
        movies_paginated_result.next_cursor_mark
    );
    logging_service::log_entry(
        "rust_create_search_index".to_string(),
        "INFO".to_string(),
        &message,
    )
        .await;

    movies_paginated_result
}

pub async fn convert_to_search_index_doc_v4(
    movies: Vec<Movie>,
    engine: Engine,
) -> Vec<SearchIndexDoc> {
    let mut docs = vec![];

    let page_size = 100;
    let mut movies_processed = 0;
    let mut end_index;
    info!("movies.len() {} ", movies.len());

    while movies_processed < movies.len() {
        if movies_processed + page_size > movies.len() {
            end_index = movies.len();
        } else {
            end_index = movies_processed + page_size;
        }

        info!(
            "processing movies {movies_processed} .. {}  of total {} movies",
            end_index,
            movies.len()
        );
        let mut principals: HashSet<String> = HashSet::new();

        for m in &movies[movies_processed..end_index] {
            principals.insert(m.tconst.clone());
        }

        let mut principals_vec = search_principals_v4(&principals, engine.clone()).await;
        info!(
            "found  {} principals for movies {:?} ",
            principals_vec.len(),
            principals
        );

        let mut persons_nconsts = HashSet::new();

        let mut principals: HashMap<String, Vec<Principal>> = HashMap::new();
        principals_vec.drain(..).for_each(|p| {
            persons_nconsts.insert(p.nconst.clone());
            let tconst = p.tconst.clone();
            if principals.contains_key(&tconst) {
                let v = principals.get_mut(&tconst).unwrap();
                v.push(p);
            } else {
                let tconst = p.tconst.clone();
                let pr = vec![p];
                principals.insert(tconst, pr);
            }
        });

        let persons_nconsts = persons_nconsts.iter().cloned().collect::<Vec<String>>();
        let mut persons_vec = search_persons_v4(persons_nconsts, engine.clone()).await;
        let mut persons: HashMap<String, Person> = HashMap::new();
        persons_vec.drain(..).for_each(|p| {
            persons.insert(p.nconst.clone(), p);
        });

        for m in &movies[movies_processed..end_index] {
            let (actors, writers, directors, characters) =
                collect_data_v4(&m.tconst, &principals, &persons);

            let (titles, characters, actors, directors, writers) =
                prepare_for_request(&m, actors, writers, directors, characters);

            let doc = SearchIndexDoc {
                id: m.tconst.clone(),
                tconst: m.tconst.clone(),
                primary_title: m.primary_title.clone(),
                original_title: m.original_title.clone(),
                titles,
                actors,
                directors,
                writers,
                runtime_minutes: m.runtime_minutes,
                adult: m.adult,
                genres: m.genres.clone(),
                characters,
                title_type: None,
                year: m.start_year,
            };
            docs.push(doc);
        }
        movies_processed += page_size;
    }

    info!(
        "docs_created.  created {} docs from a vec of {} movies",
        docs.len(),
        movies.len()
    );
    docs
}

async fn search_principals_v4(tconst: &HashSet<String>, engine: Engine) -> Vec<Principal> {
    let search_principal: String = CONFIG
        .get("search_principals_by_movie_tconst")
        .expect("expected search_principals_by_movie_tconst URL");

    let search_principal_url = search_principal.replace("ENGINE", &engine.to_string());
    let tconsts = tconst.iter().cloned().collect::<Vec<String>>();

    let search_principal_req = SearchPrincipalList { tconsts };
    let search_principal = json!(&search_principal_req);

    let message = format!("start search_principal().  url {}", search_principal_url);
    info!("message {}", &message);

    let response = CLIENT
        .post(&search_principal_url)
        .json(&search_principal)
        .send()
        .await;

    let response2 = response.unwrap();
    response2
        .json::<Vec<Principal>>()
        .await
        .expect("expected a list of principals")
}

async fn search_persons_v4(nconsts: Vec<String>, engine: Engine) -> Vec<Person> {
    let search_person_url: String = CONFIG
        .get("search_person")
        .expect("expected search_person URL");

    let search_person_url = search_person_url.replace("ENGINE", &engine.to_string());
    let search_person_req = SearchPersonList { nconsts };
    let search_persons = json!(&search_person_req);

    let response = CLIENT
        .post(search_person_url.clone())
        .header("Content-Type", "application/json".to_owned())
        .json(&search_persons)
        .send()
        .await;

    let j = json!(&search_person_req).to_string();
    dump_response_status(&response, &search_person_url, &j, engine);
    let response2 = response.unwrap();

    match response2.status().as_u16() > 300 {
        true => {
            let body = response2.text().await;
            info!("body if not status 200 {}", body.unwrap());
            vec![]
        }
        false => response2
            .json::<Vec<Person>>()
            .await
            .expect("expected a list of Persons"),
    }
}

pub fn collect_data_v4(
    tconst: &String,
    all_principals: &HashMap<String, Vec<Principal>>,
    all_persons: &HashMap<String, Person>,
) -> (
    HashSet<String>,
    HashSet<String>,
    HashSet<String>,
    HashSet<String>,
) {
    let mut actors = HashSet::new();
    let mut writers = HashSet::new();
    let mut directors = HashSet::new();
    let mut characters = HashSet::new();

    let principals = all_principals.get(tconst);

    if principals.is_none() {
        return (actors, writers, directors, characters);
    }
    let principals = principals.unwrap();

    for principal in principals {
        let person = all_persons.get(&principal.nconst);
        if person.is_some() {
            let person = person.unwrap();
            if principal.category.is_some() {
                let typ = principal.category.as_ref().unwrap();
                match typ.as_str() {
                    "actor" | "actress" => {
                        if person.primary_name.is_some() {
                            actors.insert(person.primary_name.as_ref().unwrap().clone());
                        };
                    }
                    "writer" => {
                        if person.primary_name.is_some() {
                            writers.insert(person.primary_name.as_ref().unwrap().clone());
                        };
                    }
                    "director" => {
                        if person.primary_name.is_some() {
                            directors.insert(person.primary_name.as_ref().unwrap().clone());
                        };
                    }
                    _ => {}
                };
            };
            if principal.characters.is_some() {
                let cs = principal.characters.as_ref().unwrap();
                cs.iter().for_each(|c| {
                    characters.insert(c.clone());
                });
            }
        };
    }

    (actors, writers, directors, characters)
}
