pub mod filters_search_movie {
    use std::collections::{HashMap, HashSet};
    use std::convert::Infallible;

    use reqwest::{Error, Response, StatusCode};
    use serde_json::json;
    use warp::Filter;

    use common::entity::handlers_entity::exec_meilisearch_update;
    use common::logging_service_client::logging_service;
    use common::meili_search::handlers_search_entity::dump_response_status;
    use common::movie::Movie;
    use common::person::Person;
    use common::principal::Principal;
    use common::search::{SearchPaginatedRequest, SearchPersonList};
    use common::search_doc::SearchIndexDoc;

    use crate::{CLIENT, CONFIG};

    pub fn build_index_route(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("api" / "searchindex" / "build")
            .and(warp::get())
            .and_then(|| {
                println!("GET /api/searchindex/build matched");
                build_index()
            })
    }

    pub async fn build_index() -> Result<impl warp::Reply, Infallible> {
        let mut offset = 0;
        let limit = 1000;

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
            offset += limit;

            let mut docs = vec![];
            convert_to_meilisearch_doc(total_cnt_movies, &mut cnt_movies, movies, &mut docs).await;

            let docs_json = json!(&docs).to_string();

            let message = format!(
                "sending a list of docs to the search index.  {} docs. movies processed {} / {}",
                docs.len(),
                cnt_movies,
                total_cnt_movies
            );
            println!("{}", &message);

            logging_service::log_entry(
                "rust_create_search_index".to_string(),
                "INFO".to_string(),
                &message,
            )
            .await;

            println!("starting update request for  {} docs", docs.len());
            exec_meilisearch_update(&"searchindex".to_string(), &CLIENT, docs_json).await;
            println!(
                "finished update request for  {} docs.  . movies processed {} / {} ",
                docs.len(),
                cnt_movies,
                total_cnt_movies
            );
        }

        let message = format!("finished build_index(). processed {} movies ", cnt_movies);
        println!("res {}", &message);
        logging_service::log_entry(
            "rust_create_search_index".to_string(),
            "INFO".to_string(),
            &message,
        )
        .await;
        println!("done {}", &message);
        Ok(warp::reply::json(&message))
    }

    async fn convert_to_meilisearch_doc(
        total_cnt_movies: i32,
        cnt_movies: &mut i32,
        movies: Vec<Movie>,
        docs: &mut Vec<SearchIndexDoc>,
    ) {
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

            println!(
                "processing movie tconst: {}.    movie {} / {}  ",
                m.tconst, cnt_movies, total_cnt_movies
            );
            *cnt_movies += 1;
        }
    }

    fn prepare_for_request(
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

    fn collect_data(
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
                        None => println!(
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

    async fn search_movies(limit: u32, offset: u32) -> Vec<Movie> {
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
        println!("message {}", &message);
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
        println!("message {}", &message);
        logging_service::log_entry(
            "rust_create_search_index".to_string(),
            "INFO".to_string(),
            &message,
        )
        .await;

        // let _movies_as_pretty_json = serde_json::to_string_pretty(&movies).unwrap();
        // println!("got a list of movies {}", movies_as_pretty_json);
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
                    println!("{} != OK.  returned HTTP code {} ", msg1, code);
                    println!(
                        "{} != OK.  returned HTTP code {} headers {:?}",
                        msg1, code, x
                    );

                    println!("message {}", &message);
                    logging_service::log_entry(
                        "rust_create_search_index".to_string(),
                        "ERROR".to_string(),
                        message,
                    )
                    .await;
                }
            }
            Err(e) => println!("error in request to meilisearch {:?}", e),
        };
    }

    async fn search_principal(tconst: &String) -> Vec<Principal> {
        let search_principal: String = CONFIG
            .get("search_principal_by_movie_tconst")
            .expect("expected search_principal_by_movie_tconst URL");

        let url = format!("{search_principal}{tconst}");
        //   println!("searching principals for movie tconst {tconst}. search url {url}");

        let message = format!("start search_principal().  url {}", url);
        println!("message {}", &message);
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
        //   println!("got a list of principals {}", &principals_as_pretty_json);

        let message = format!(
            "end search_principal().  url {}. found {} prinicpals",
            &url,
            principals.len()
        );
        println!("message {}", &message);
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

        //   println!("sending request to url {},   payload {}", search_person_url, search_persons);

        let response = CLIENT
            .post(search_person_url)
            .header("Content-Type", "application/json".to_owned())
            .json(&search_persons)
            .send()
            .await;

        dump_response_status(&response);
        let response2 = response.unwrap();

        match response2.status().as_u16() > 300 {
            true => {
                let body = response2.text().await;
                println!("body if not status 200 {}", body.unwrap());
                //
                // let persons = response2
                //     .json::<Vec<Person>>()
                //     .await
                //     .expect("expected a list of Persons");
                //
                // let persons_as_pretty_json = serde_json::to_string_pretty(&persons).unwrap();
                // println!("got a list of persons {}", persons_as_pretty_json);

                // persons
                vec![]
            }
            false => {
                let persons = response2
                    .json::<Vec<Person>>()
                    .await
                    .expect("expected a list of Persons");

                //  let persons_as_pretty_json = serde_json::to_string_pretty(&persons).unwrap();
                //  println!("got a list of persons {}", persons_as_pretty_json);

                persons
            }
        }
    }

    // async fn search_crew(tconst: &String) -> Vec<Crew> {
    //     let search_crew_url: String = CONFIG
    //         .get("search_crew_by_tconst")
    //         .expect("expected search_crew_by_tconst  URL");
    //
    //     let url = format!("{search_crew_url}{tconst}");
    //     //   println!("searching crew for movie tconst {tconst}. search url {url}");
    //
    //     let response = CLIENT.get(url).send().await;
    //
    //     dump_response_status(&response);
    //
    //     let response2 = response.unwrap();
    //     let crew = response2
    //         .json::<Vec<Crew>>()
    //         .await
    //         .expect("expected a list of Crew");
    //
    //    //  let crew_as_pretty_json = serde_json::to_string_pretty(&crew).unwrap();
    //     //     println!("got a list of crew {}", crew_as_pretty_json);
    //
    //     crew
    // }
}
