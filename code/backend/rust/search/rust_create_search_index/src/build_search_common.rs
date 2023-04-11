use std::collections::{HashMap, HashSet};

use log::{error, info};
use reqwest::{Error, Response, StatusCode};
use serde_json::json;

use common::logging::logging_service_client::logging_service;
use common::meili::dump_response_status;
use common::models::movie::Movie;
use common::models::person::{Person, SearchPersonList};
use common::models::principal::Principal;
use common::models::search_doc::{SearchIndexDoc, SearchPaginatedRequest};

use crate::{CLIENT, CONFIG};

pub async fn convert_to_search_index_doc(
    movies: Vec<Movie>,
    docs: &mut Vec<SearchIndexDoc>,
    engine: String,
) {
    for m in movies {
        let principals = search_principal(&m.tconst.clone(), engine.clone()).await;
        // let crew = search_crew(&m.tconst).await;

        let mut person_nconsts: HashSet<String> = HashSet::new();

        principals.iter().for_each(|p| {
            let _ = person_nconsts.insert(p.nconst.clone());
        });
        let vec1 = person_nconsts.iter().cloned().collect::<Vec<String>>();

        let mut persons_vec = search_person(vec1, engine.clone()).await;
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
            title_type: None,
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

pub async fn search_movies(limit: u32, offset: u32, engine: String) -> Vec<Movie> {
    info!("search_movies");
    let search_movie: String = CONFIG
        .get("search_movie")
        .expect("expected search_movie URL");

    let search_movie = search_movie.replace("ENGINE", &engine);

    let search_request = SearchPaginatedRequest {
        q: "*".to_string(),
        offset,
        limit,
        sort: vec!["tconst:asc".to_string()],
    };

    let message = format!(
        "start search_movies().  offset {}, limit {}, sort {:?}, engine: {}",
        offset,
        limit,
        &search_request.sort.clone(),
        engine.clone()
    );
    info!("message {}", &message);
    logging_service::log_entry(
        "rust_create_search_index".to_string(),
        "INFO".to_string(),
        &message,
    )
        .await;

    info!("search movie URL {}", &search_movie);
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
        "XXX    search_movies all good. found {} movies",
        movies.len()
    );

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
    info!("XXX    search_movies finished succesfully");

    // let _movies_as_pretty_json = serde_json::to_string_pretty(&movies).unwrap();
    // info!("got a list of movies {}", movies_as_pretty_json);
    movies
}

async fn search_principal(tconst: &String, engine: String) -> Vec<Principal> {
    let search_principal: String = CONFIG
        .get("search_principal_by_movie_tconst")
        .expect("expected search_principal_by_movie_tconst URL");

    let search_principal = search_principal.replace("ENGINE", &engine);

    let url = format!("{search_principal}{tconst}");
    info!("searching principals for movie tconst {tconst}. engine {engine}.  search url '{url}'");

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

async fn search_person(nconsts: Vec<String>, engine: String) -> Vec<Person> {
    let search_person_url: String = CONFIG
        .get("search_person")
        .expect("expected search_person URL");

    let search_person_url = search_person_url.replace("ENGINE", &engine);

    let search_person_req = SearchPersonList { nconsts };

    let search_persons = json!(&search_person_req);
    let tmp = json!(&search_person_req).to_string();

    info!(
        "sending request to engine '{}' to url '{}'.  payload '{}'",
        engine, search_person_url, &tmp
    );

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
