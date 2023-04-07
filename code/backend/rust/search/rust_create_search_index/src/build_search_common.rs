use std::collections::{HashMap, HashSet};

use reqwest::{Error, Response, StatusCode};
use serde_json::json;

use common::logging_service_client::logging_service;
use common::meili_search::handlers_search_entity::dump_response_status;
use common::movie::Movie;
use common::person::Person;
use common::principal::Principal;
use common::search::{SearchPaginatedRequest, SearchPersonList};
use common::search_doc::SearchIndexDoc;

use crate::{CLIENT, CONFIG};

pub async fn convert_to_meilisearch_doc(movies: Vec<Movie>, docs: &mut Vec<SearchIndexDoc>) {
    for m in movies {
        let principals = search_principal(&m.tconst.clone()).await;
        // let crew = search_crew(&m.tconst).await;

        let mut person_nconsts: HashSet<String> = HashSet::new();

        principals.iter().for_each(|p| {
            let _ = person_nconsts.insert(p.nconst.clone());
        });
        let vec1 = person_nconsts.iter().cloned().collect::<Vec<String>>();

        let mut persons_vec = search_person(vec1).await;
        let mut persons = HashMap::new();
        persons_vec.drain(..).for_each(|p| {
            let id = p.nconst.clone();
            persons.insert(id, p);
        });

        let mut actors: Vec<String> = vec![];
        let mut writers: Vec<String> = vec![];
        let mut directors: Vec<String> = vec![];
        let mut characters: Vec<String> = vec![];

        collect_data(
            principals,
            persons,
            &mut actors,
            &mut writers,
            &mut directors,
            &mut characters,
        );

        let (titles, characters, actors, directors, writers) =
            prepare_for_request(&m, actors, writers, directors, characters);

        let doc = SearchIndexDoc {
            id: m.tconst.clone(),
            tconst: m.tconst.clone(),
            titles,
            actors,
            directors,
            writers,
            runtime_minutes: m.runtime_minutes,
            adult: m.adult,
            genres: m.genres.clone(),
            characters,
        };
        docs.push(doc);

        info!("processing movie tconst: {}.", m.tconst);
    }
}

pub fn prepare_for_request(
    m: &Movie,
    actors: Vec<String>,
    writers: Vec<String>,
    directors: Vec<String>,
    characters: Vec<String>,
) -> (
    Option<Vec<String>>,
    Option<Vec<String>>,
    Option<Vec<String>>,
    Option<Vec<String>>,
    Option<Vec<String>>,
) {
    let mut titles = HashSet::new();
    if m.original_title.is_some() {
        // funny demo
        // https://github.com/rust-lang/rust-clippy/issues/9064 f
        let s = m.original_title.as_ref().map(String::to_string).unwrap();
        titles.insert(s);
    }
    if m.primary_title.is_some() {
        let s = m.primary_title.as_ref().map(String::to_string).unwrap();
        titles.insert(s);
    }

    let titles = titles.into_iter().collect::<Vec<String>>();

    // map to optionals
    let titles = match titles.len() > 0 {
        true => Some(titles),
        false => None,
    };
    let characters = match characters.len() > 0 {
        true => Some(characters),
        false => None,
    };
    let actors = match actors.len() > 0 {
        true => Some(actors),
        false => None,
    };
    let directors = match directors.len() > 0 {
        true => Some(directors),
        false => None,
    };

    let writers = match writers.len() > 0 {
        true => Some(writers),
        false => None,
    };
    (titles, characters, actors, directors, writers)
}

pub fn collect_data(
    principals: Vec<Principal>,
    persons: HashMap<String, Person>,
    actors: &mut Vec<String>,
    writers: &mut Vec<String>,
    directors: &mut Vec<String>,
    characters: &mut Vec<String>,
) {
    principals.iter().for_each(|p| {
        match &p.category {
            Some(typ) => match typ.as_str() {
                "actor" | "actress" => {
                    let a = persons.get(&p.nconst).unwrap();
                    match &a.primary_name {
                        Some(name) => actors.push(name.clone()),
                        None => {}
                    }
                }
                "writer" => {
                    let a = persons.get(&p.nconst).unwrap();
                    match &a.primary_name {
                        Some(name) => writers.push(name.clone()),
                        None => {}
                    }
                }
                "director" => match persons.get(&p.nconst) {
                    Some(per) => match &per.primary_name {
                        Some(name) => directors.push(name.clone()),
                        None => {}
                    },
                    None => info!(
                        "hm - why is there no person?, principal {} and director",
                        p.id
                    ),
                },
                _ => {}
            },
            None => {}
        };
        match &p.characters {
            Some(ch) => {
                ch.iter().for_each(|c| {
                    characters.push(c.clone());
                });
            }
            None => {}
        }
    });
}

pub async fn search_movies(limit: u32, offset: u32) -> Vec<Movie> {
    let search_movie: String = CONFIG
        .get("search_movie")
        .expect("expected search_movie URL");

    let search_request = SearchPaginatedRequest {
        q: "*".to_string(),
        offset,
        limit,
        sort: vec!["tconst:asc".to_string()],
    };

    let message = format!(
        "start search_movies().  offset {}, limit {}, sort {:?} ",
        offset,
        limit,
        &search_request.sort.clone()
    );
    info!("message {}", &message);
    logging_service::log_entry(
        "rust_create_search_index".to_string(),
        "INFO".to_string(),
        &message,
    )
    .await;

    let json = json!(&search_request);
    let response = CLIENT.post(search_movie).json(&json).send().await;

    let message = format!(
        "error search_movies(). offset {}, limit {}, sort {:?}.",
        offset,
        limit,
        &search_request.sort.clone()
    );
    let msg = "search for movies paginated search request".to_string();
    log_external_service_error(&msg, &message, &response).await;

    let response2 = response.unwrap();
    let movies = response2
        .json::<Vec<Movie>>()
        .await
        .expect("expected a list of Movies");

    let message = format!(
        "end search_movies().  offset {}, limit {}, sort {:?}. {} movies found ",
        offset,
        limit,
        &search_request.sort.clone(),
        movies.len()
    );
    info!("message {}", &message);
    logging_service::log_entry(
        "rust_create_search_index".to_string(),
        "INFO".to_string(),
        &message,
    )
    .await;

    // let _movies_as_pretty_json = serde_json::to_string_pretty(&movies).unwrap();
    // info!("got a list of movies {}", movies_as_pretty_json);
    movies
}

async fn log_external_service_error(
    msg1: &String,
    message: &String,
    response: &Result<Response, Error>,
) {
    match &response {
        Ok(res) => {
            let code = res.status();
            if code != StatusCode::OK {
                let x = res.headers().clone();
                // let b = res.text().await.unwrap();
                info!("{} != OK.  returned HTTP code {} ", msg1, code);
                info!(
                    "{} != OK.  returned HTTP code {} headers {:?}",
                    msg1, code, x
                );

                info!("message {}", &message);
                logging_service::log_entry(
                    "rust_create_search_index".to_string(),
                    "ERROR".to_string(),
                    message,
                )
                .await;
            }
        }
        Err(e) => error!("error in request to meilisearch {:?}", e),
    };
}

async fn search_principal(tconst: &String) -> Vec<Principal> {
    let search_principal: String = CONFIG
        .get("search_principal_by_movie_tconst")
        .expect("expected search_principal_by_movie_tconst URL");

    let url = format!("{search_principal}{tconst}");
    //   info!("searching principals for movie tconst {tconst}. search url {url}");

    let message = format!("start search_principal().  url {}", url);
    info!("message {}", &message);
    logging_service::log_entry(
        "rust_create_search_index".to_string(),
        "INFO".to_string(),
        &message,
    )
    .await;

    let response = CLIENT.get(&url).send().await;

    let message = format!(
        "error search_principal(). an error occurred in requesting a list of principals. url {}  ",
        &url
    );
    let msg = "search for principal search request".to_string();
    log_external_service_error(&msg, &message, &response).await;

    let response2 = response.unwrap();
    let principals = response2
        .json::<Vec<Principal>>()
        .await
        .expect("expected a list of principals");

    // let principals_as_pretty_json = serde_json::to_string_pretty(&principals).unwrap();
    //   info!("got a list of principals {}", &principals_as_pretty_json);

    let message = format!(
        "end search_principal().  url {}. found {} prinicpals",
        &url,
        principals.len()
    );
    info!("message {}", &message);
    logging_service::log_entry(
        "rust_create_search_index".to_string(),
        "INFO".to_string(),
        &message,
    )
    .await;

    principals
}

async fn search_person(nconsts: Vec<String>) -> Vec<Person> {
    let search_person_url: String = CONFIG
        .get("search_person_by_nconst")
        .expect("expected search_person_by_nconst URL");

    let search_person_req = SearchPersonList { nconsts };

    let search_persons = json!(&search_person_req);

    //   info!("sending request to url {},   payload {}", search_person_url, search_persons);

    let response = CLIENT
        .post(search_person_url.clone())
        .header("Content-Type", "application/json".to_owned())
        .json(&search_persons)
        .send()
        .await;

    dump_response_status(&response, &search_person_url, &"none available".to_string());
    let response2 = response.unwrap();

    match response2.status().as_u16() > 300 {
        true => {
            let body = response2.text().await;
            info!("body if not status 200 {}", body.unwrap());
            vec![]
        }
        false => {
            let persons = response2
                .json::<Vec<Person>>()
                .await
                .expect("expected a list of Persons");

            //  let persons_as_pretty_json = serde_json::to_string_pretty(&persons).unwrap();
            //  info!("got a list of persons {}", persons_as_pretty_json);

            let message = format!(
                "end search_person().  url {}. found {} persons",
                &search_person_url,
                persons.len()
            );
            info!("message {}", &message);
            logging_service::log_entry(
                "rust_create_search_index".to_string(),
                "INFO".to_string(),
                &message,
            )
            .await;

            persons
        }
    }
}
