pub mod filters_search_search_index {
    use std::collections::{HashMap, HashSet};
    use std::convert::Infallible;

    use reqwest::StatusCode;
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

    pub fn search_index_route() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
        warp::path!("api" / "searchindex" / "search")
            .and(warp::post())
            .and_then(|| {
                info!("POST /api/searchindex/search matched");
                search_index()
            })
    }

    pub async fn search_index() -> Result<impl warp::Reply, Infallible> {
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
            cnt_movies += movies.len();

            let mut docs = vec![];
            for m in movies {
                let principals = search_principal(&m.tconst.clone()).await;
                // let crew = search_crew(&m.tconst).await;

                let mut person_nconsts: HashSet<String> = HashSet::new();

                principals.iter().for_each(|p| {
                    let _ = person_nconsts.insert(p.nconst.clone());
                });
                // crew.iter().for_each(|c| {
                //     match &c.directors {
                //         Some(dirs) => {
                //             dirs.iter().for_each(|d| {
                //                 let _ = person_nconsts.insert(d.clone());
                //             });
                //         }
                //         None => {}
                //     }
                //     match &c.writers {
                //         Some(writ) => {
                //             writ.iter().for_each(|d| {
                //                 let _ = person_nconsts.insert(d.clone());
                //             });
                //         }
                //         None => {}
                //     }
                // });
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
                                None => error!(
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

                info!(
                    "processing movie tconst: {}.    movie {} / {}  ",
                    m.tconst, cnt_movies, total_cnt_movies
                );
            }

            let docs_json = json!(&docs).to_string();
            info!(
                "sending a list of docs to the search index.  {} docs. movies processed {} / {}",
                docs.len(),
                cnt_movies,
                total_cnt_movies
            );

            info!("starting update request for  {} docs", docs.len());
            exec_meilisearch_update(&"searchindex".to_string(), &CLIENT, docs_json).await;
            info!(
                "finished update request for  {} docs.  . movies processed {} / {} ",
                docs.len(),
                cnt_movies,
                total_cnt_movies
            );
        }

        let res = format!("finished build_index(). processed {} movies ", cnt_movies);

        info!("res {}", &res);
        logging_service::log_entry(
            "rust_create_search_index".to_string(),
            "INFO".to_string(),
            &res,
        )
            .await;
        info!("done {}", &res);
        Ok(warp::reply::json(&res))
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

        let json = json!(&search_request);
        let response = CLIENT.post(search_movie).json(&json).send().await;

        match &response {
            Ok(res) => {
                let code = res.status().clone();
                if code == StatusCode::OK {
                    info!("search for movies paginated search request success");
                } else {
                    let x = res.headers().clone();
                    // let b = res.text().await.unwrap();
                    error!(
                        "search for movies paginated search request != OK. status {:?}",
                        code
                    );
                    error!(
                        "search for movies paginated search request != OK. headers {:?}",
                        x
                    );
                    // error!("meilisearch search request != OK. response body {:?}", &b);
                }
            }
            Err(e) => error!("error in request to meilisearch {:?}", e),
        };

        let response2 = response.unwrap();
        let movies = response2
            .json::<Vec<Movie>>()
            .await
            .expect("expected a list of Movies");

        // let _movies_as_pretty_json = serde_json::to_string_pretty(&movies).unwrap();
        // info!("got a list of movies {}", movies_as_pretty_json);
        movies
    }

    async fn search_principal(tconst: &String) -> Vec<Principal> {
        let search_principal: String = CONFIG
            .get("search_principal_by_movie_tconst")
            .expect("expected search_principal_by_movie_tconst URL");

        let url = format!("{search_principal}{tconst}");
        //   info!("searching principals for movie tconst {tconst}. search url {url}");

        let response = CLIENT.get(url).send().await;

        match &response {
            Ok(res) => {
                let code = res.status().clone();
                if code == StatusCode::OK {
                    info!("search for principal   search request success");
                } else {
                    let x = res.headers().clone();
                    // let b = res.text().await.unwrap();
                    error!(
                        "search for principal search request != OK. status {:?}",
                        code
                    );
                    error!("search for principal search request != OK. headers {:?}", x);
                    // info!("meilisearch search request != OK. response body {:?}", &b);
                }
            }
            Err(e) => error!("error in request to meilisearch {:?}", e),
        };

        let response2 = response.unwrap();
        let principals = response2
            .json::<Vec<Principal>>()
            .await
            .expect("expected a list of principals");

        // let principals_as_pretty_json = serde_json::to_string_pretty(&principals).unwrap();
        //   info!("got a list of principals {}", &principals_as_pretty_json);

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
                error!("body if not status 200 {}", body.unwrap());
                //
                // let persons = response2
                //     .json::<Vec<Person>>()
                //     .await
                //     .expect("expected a list of Persons");
                //
                // let persons_as_pretty_json = serde_json::to_string_pretty(&persons).unwrap();
                // info!("got a list of persons {}", persons_as_pretty_json);

                // persons
                vec![]
            }
            false => {
                let persons = response2
                    .json::<Vec<Person>>()
                    .await
                    .expect("expected a list of Persons");

                //  let persons_as_pretty_json = serde_json::to_string_pretty(&persons).unwrap();
                //  info!("got a list of persons {}", persons_as_pretty_json);

                persons
            }
        }
    }
}
