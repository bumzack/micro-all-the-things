use std::collections::{HashMap, HashSet};

use log::{error, info};
use serde_json::json;

use common::entity::entity::Engine;
use common::helper::dump_response_status;
use common::models::movie::{Movie, MoviePaginationResult};
use common::models::person::{Person, SearchPersonList};
use common::models::principal::Principal;
use common::models::search_doc::{SearchIndexDoc, SearchPaginatedRequest};

use crate::{CLIENT, CONFIG};

pub async fn convert_to_search_index_doc(
    movies: Vec<Movie>,
    docs: &mut Vec<SearchIndexDoc>,
    engine: Engine,
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

        let mut actors: HashSet<String> = HashSet::new();
        let mut writers: HashSet<String> = HashSet::new();
        let mut directors: HashSet<String> = HashSet::new();
        let mut characters: HashSet<String> = HashSet::new();

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
            primary_title: m.primary_title,
            original_title: m.original_title,
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
}

pub fn prepare_for_request(
    m: &Movie,
    actors: HashSet<String>,
    writers: HashSet<String>,
    directors: HashSet<String>,
    characters: HashSet<String>,
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
    let titles = match !titles.is_empty() {
        true => Some(titles),
        false => None,
    };

    let characters = match !characters.is_empty() {
        true => Some(characters.into_iter().collect::<Vec<String>>()),
        false => None,
    };

    let actors = match !actors.is_empty() {
        true => Some(actors.into_iter().collect::<Vec<String>>()),
        false => None,
    };

    let directors = match !directors.is_empty() {
        true => Some(directors.into_iter().collect::<Vec<String>>()),
        false => None,
    };

    let writers = match !writers.is_empty() {
        true => Some(writers.into_iter().collect::<Vec<String>>()),
        false => None,
    };

    (titles, characters, actors, directors, writers)
}

pub fn collect_data(
    principals: Vec<Principal>,
    persons: HashMap<String, Person>,
    actors: &mut HashSet<String>,
    writers: &mut HashSet<String>,
    directors: &mut HashSet<String>,
    characters: &mut HashSet<String>,
) {
    principals.iter().for_each(|p| {
        match &p.category {
            Some(typ) => match typ.as_str() {
                "actor" | "actress" => {
                    let a = persons.get(&p.nconst).unwrap();
                    match &a.primary_name {
                        Some(name) => {
                            let _ = actors.insert(name.clone());
                        }
                        None => {}
                    }
                }
                "writer" => {
                    let a = persons.get(&p.nconst).unwrap();
                    match &a.primary_name {
                        Some(name) => {
                            let _ = writers.insert(name.clone());
                        }
                        None => {}
                    }
                }
                "director" => match persons.get(&p.nconst) {
                    Some(per) => match &per.primary_name {
                        Some(name) => {
                            let _ = directors.insert(name.clone());
                        }
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
                    characters.insert(c.clone());
                });
            }
            None => {}
        }
    });
}

pub async fn search_movies(limit: u32, offset: u32, engine: Engine) -> Vec<Movie> {
    let search_movie: String = CONFIG
        .get("search_movie")
        .expect("expected search_movie URL");

    let search_movie = search_movie.replace("ENGINE", &engine.to_string());

    let search_request = SearchPaginatedRequest {
        q: "*".to_string(),
        offset,
        limit,
        sort: vec!["tconst:asc".to_string()],
        next_cursor_mark: None,
    };

    let message = format!(
        "start search_movies().  offset {}, limit {}, sort {:?}, engine: {}",
        offset,
        limit,
        &search_request.sort.clone(),
        engine.to_string()
    );
    info!("message {}", &message);

    let json = json!(&search_request);
    let response = CLIENT.post(search_movie).json(&json).send().await;

    // let message = format!(
    //     "error search_movies(). offset {}, limit {}, sort {:?}.",
    //     offset,
    //     limit,
    //     &search_request.sort.clone()
    // );
    // let msg = "search for movies paginated search request".to_string();
    // log_external_service_error(&msg, &message, &response).await;

    if response.is_err() {
        error!(
            "error from SearchMovie Service {:?}",
            response.err().unwrap()
        );
        return vec![];
    }

    let response2 = response.unwrap();
    let movies_paginated_result = response2
        .json::<MoviePaginationResult>()
        .await
        .expect("expected a list of Movies");

    let _message = format!(
        "end search_movies().  offset {}, limit {}, sort {:?}. {} movies found. next_cursor_mark {:?} ",
        offset,
        limit,
        &search_request.sort.clone(),
        movies_paginated_result.movies.len(),
        movies_paginated_result.next_cursor_mark
    );

    movies_paginated_result.movies
}

async fn search_principal(tconst: &String, engine: Engine) -> Vec<Principal> {
    let search_principal: String = CONFIG
        .get("search_principal_by_movie_tconst")
        .expect("expected search_principal_by_movie_tconst URL");

    let search_principal = search_principal.replace("ENGINE", &engine.to_string());

    let url = format!("{search_principal}{tconst}");

    let message = format!("start search_principal().  url {}", url);
    info!("message {}", &message);

    let response = CLIENT.get(&url).send().await;

    // let message = format!(
    //     "error search_principal(). an error occurred in requesting a list of principals. url {}  ",
    //     &url
    // );
    // let msg = "search for principal search request".to_string();
    // log_external_service_error(&msg, &message, &response).await;

    let response2 = response.unwrap();
    response2
        .json::<Vec<Principal>>()
        .await
        .expect("expected a list of principals")
}

async fn search_person(nconsts: Vec<String>, engine: Engine) -> Vec<Person> {
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
