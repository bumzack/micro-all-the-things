use log::{error, info};
use reqwest::{Error, Response, StatusCode};

pub mod search_entity_paginated {
    use log::info;
    use reqwest::{Client, Error, Response};
    use serde_json::json;

    use crate::meili_search::dump_response_status;
    use crate::search::MeiliSearchRequest;

    pub(crate) async fn meili_search_paginated(
        entity: String,
        search_text: String,
        limit: u32,
        offset: u32,
        facets: Vec<String>,
        client: &Client,
    ) -> Result<Response, Error> {
        info!(
            "searching for entity {} and search_term '{}'",
            &entity, &search_text
        );

        let search_request = MeiliSearchRequest {
            q: search_text,
            offset: Some(offset),
            limit: Some(limit),
            page: None,
            hits_per_page: Some(limit),
            filter: None,
            facets: Some(facets),
            attributes_to_retrieve: None,
            attributes_to_crop: None,
            crop_marker: None,
            crop_length: None,
            attributes_to_highlight: None,
            highlight_pre_tag: None,
            highlight_post_tag: None,
            show_matches_position: None,
            sort: None,
            matching_strategy: None,
        };

        let json = json!(&search_request).to_string();
        info!("request body {}", &json);
        let index = format!("http://meilisearch01.bumzack.at/indexes/{}/search", &entity);

        let response = client
            .post(&index)
            .body(json.clone())
            .header("Authorization", "Bearer 1234567890123456".to_owned())
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        dump_response_status(&response, &index, &json);

        response
    }
}

pub mod meili_search_searchindex {
    use std::convert::Infallible;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::meili_search::search_entity_paginated::meili_search_paginated;
    use crate::search::MeiliSearchResult;
    use crate::search_doc::SearchIndexDoc;

    pub async fn meili_search_searchindex(
        entity: String,
        search_text: String,
        limit: u32,
        offset: u32,
        facets: Vec<String>,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_search_paginated(
            entity,
            search_text.clone(),
            limit,
            offset,
            facets.clone(),
            client,
        );

        let response2 = response.await;
        let docs = match response2 {
            Ok(r) => {
                let code = r.status();
                if code == StatusCode::OK
                    || code == StatusCode::ACCEPTED
                    || code == StatusCode::CREATED
                {
                    info!(
                        "meili_search_searchindex request success. unwrapping MeiliSearchResult<SearchIndexDoc>"
                    );
                    let result = r.json::<MeiliSearchResult<SearchIndexDoc>>().await;
                    match result {
                        Ok(r) => {
                            info!("meili_search_searchindex request success and all good. returning Vec<SearchIndexDoc>");
                            r.hits
                        }
                        Err(ee) => {
                            info!("meili_search_searchindex request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!(
                        "meili_search_searchindex request != OK. status {:?},     ",
                        code
                    );
                    error!(
                        "meili_search_searchindex request != OK. headers {:?},   ",
                        x
                    );
                    vec![]
                }
            }
            Err(eee) => {
                error!("meili_search_searchindex wtf?. error {:?}", eee);
                vec![]
            }
        };
        Ok(warp::reply::json(&docs))
    }
}

pub mod meili_search_person {
    use std::convert::Infallible;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::meili_search::meili_search::meili_search;
    use crate::person::Person;
    use crate::search::MeiliSearchResult;

    pub async fn meili_search_person(
        entity: String,
        search_text: String,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_search(entity, search_text, client);

        let response2 = response.await;
        let persons = match response2 {
            Ok(r) => {
                let code = r.status();
                if code == StatusCode::OK
                    || code == StatusCode::ACCEPTED
                    || code == StatusCode::CREATED
                {
                    info!(
                        "meili_search_person request success. unwrapping MeiliSearchResult<Person>"
                    );
                    let result = r.json::<MeiliSearchResult<Person>>().await;
                    match result {
                        Ok(r) => {
                            info!("meili_search_person request success and all good. returning Vec<Person>");
                            r.hits
                        }
                        Err(ee) => {
                            info!("meili_search_person request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!("meili_search_person request != OK. status {:?},     ", code);
                    error!("meili_search_person request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("meili_search_person wtf?. error {:?}", eee);
                vec![]
            }
        };

        Ok(warp::reply::json(&persons))
    }
}

pub mod meili_search_movie {
    use std::convert::Infallible;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::logging_service_client::logging_service;
    use crate::meili_search::meili_search::meili_search;
    use crate::movie::Movie;
    use crate::search::MeiliSearchResult;

    pub async fn meili_search_movie(
        entity: String,
        search_text: String,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_search(entity, search_text.clone(), client);

        let response2 = response.await;
        let movies = match response2 {
            Ok(r) => {
                let code = r.status();
                if code == StatusCode::OK
                    || code == StatusCode::ACCEPTED
                    || code == StatusCode::CREATED
                {
                    info!(
                        "meili_search_movie request success. unwrapping MeiliSearchResult<Movie>"
                    );
                    let result = r.json::<MeiliSearchResult<Movie>>().await;
                    match result {
                        Ok(r) => {
                            info!("meili_search_movie request success and all good. returning Vec<Movie>");
                            r.hits
                        }
                        Err(ee) => {
                            info!("meili_search_movie request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!("meili_search_movie request != OK. status {:?},     ", code);
                    error!("meili_search_movie request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("meili_search_movie wtf?. error {:?}", eee);
                vec![]
            }
        };

        let msg = format!(
            "finished meili_search_movie(). search_text '{}' returned {} movies ",
            search_text,
            movies.len()
        );
        logging_service::log_entry(
            "rust_create_search_index".to_string(),
            "INFO".to_string(),
            &msg,
        )
            .await;

        Ok(warp::reply::json(&movies))
    }
}

pub mod meili_search_movieaka {
    use std::convert::Infallible;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::meili_search::meili_search::meili_search;
    use crate::movieaka::MovieAkas;
    use crate::search::MeiliSearchResult;

    pub async fn meili_search_movieaka(
        entity: String,
        search_text: String,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_search(entity, search_text, client);

        let response2 = response.await;
        let movieakas = match response2 {
            Ok(r) => {
                let code = r.status();
                if code == StatusCode::OK
                    || code == StatusCode::ACCEPTED
                    || code == StatusCode::CREATED
                {
                    info!(
                        "meili_search_movieaka request success. unwrapping MeiliSearchResult<MovieAkas>"
                    );
                    let result = r.json::<MeiliSearchResult<MovieAkas>>().await;
                    match result {
                        Ok(r) => {
                            info!("meili_search_movieaka request success and all good. returning Vec<MovieAkas>");
                            r.hits
                        }
                        Err(ee) => {
                            info!("meili_search_movieaka request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!(
                        "meili_search_movieaka request != OK. status {:?},     ",
                        code
                    );
                    error!("meili_search_movieaka request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("meili_search_movieakaaka wtf?. error {:?}", eee);
                vec![]
            }
        };

        Ok(warp::reply::json(&movieakas))
    }
}

pub mod meili_search_crew {
    use std::convert::Infallible;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::crew::Crew;
    use crate::meili_search::meili_search::meili_search;
    use crate::search::MeiliSearchResult;

    pub async fn meili_search_crew(
        entity: String,
        search_text: String,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_search(entity, search_text, client);

        let response2 = response.await;
        let crew = match response2 {
            Ok(r) => {
                let code = r.status();
                if code == StatusCode::OK
                    || code == StatusCode::ACCEPTED
                    || code == StatusCode::CREATED
                {
                    info!("meili_search_crew request success. unwrapping MeiliSearchResult<Crew>");
                    let result = r.json::<MeiliSearchResult<Crew>>().await;
                    match result {
                        Ok(r) => {
                            info!("meili_search_crew request success and all good. returning Vec<Crew>");
                            r.hits
                        }
                        Err(ee) => {
                            info!("meili_search_crew request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!("meili_search_crew request != OK. status {:?},     ", code);
                    error!("meili_search_crew request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("meili_search_crew wtf?. error {:?}", eee);
                vec![]
            }
        };

        Ok(warp::reply::json(&crew))
    }
}

pub mod meili_search_episode {
    use std::convert::Infallible;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::meili_search::meili_search::meili_search;
    use crate::rating::Rating;
    use crate::search::MeiliSearchResult;

    pub async fn meili_search_episode(
        entity: String,
        search_text: String,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_search(entity, search_text, client);

        let response2 = response.await;
        let ratings = match response2 {
            Ok(r) => {
                let code = r.status();
                if code == StatusCode::OK
                    || code == StatusCode::ACCEPTED
                    || code == StatusCode::CREATED
                {
                    info!(
                        "meili_search_rating request success. unwrapping MeiliSearchResult<Rating>"
                    );
                    let result = r.json::<MeiliSearchResult<Rating>>().await;
                    match result {
                        Ok(r) => {
                            info!("meili_search_rating request success and all good. returning Vec<Rating>");
                            r.hits
                        }
                        Err(ee) => {
                            info!("meili_search_rating request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!("meili_search_rating request != OK. status {:?},     ", code);
                    error!("meili_search_rating request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("meili_search_rating wtf?. error {:?}", eee);
                vec![]
            }
        };

        Ok(warp::reply::json(&ratings))
    }
}

pub mod meili_search {
    use log::info;
    use reqwest::{Client, Error, Response};
    use serde_json::json;

    use crate::meili_search::dump_response_status;
    use crate::search::MeiliSearchRequest;

    pub(crate) async fn meili_search(
        entity: String,
        search_text: String,
        client: &Client,
    ) -> Result<Response, Error> {
        info!(
            "searching for entity {} and search_term '{}'",
            &entity, &search_text
        );

        let search_request = MeiliSearchRequest {
            q: search_text,
            offset: None,
            limit: None,
            page: None,
            hits_per_page: None,
            filter: None,
            facets: None,
            attributes_to_retrieve: None,
            attributes_to_crop: None,
            crop_marker: None,
            crop_length: None,
            attributes_to_highlight: None,
            highlight_pre_tag: None,
            highlight_post_tag: None,
            show_matches_position: None,
            sort: None,
            matching_strategy: None,
        };

        let json = json!(&search_request).to_string();
        let index = format!("http://meilisearch01.bumzack.at/indexes/{}/search", &entity);

        info!(
            "searching for entity of type {}. POST to url {}   json {}",
            &entity, &index, &json
        );

        let response = client
            .post(&index)
            .body(json.clone())
            .header("Authorization", "Bearer 1234567890123456".to_owned())
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        dump_response_status(&response, &index, &json);

        response
    }
}

pub mod meili_search_movie_paginated {
    use std::convert::Infallible;

    use log::{error, info};
    use reqwest::{Client, StatusCode};
    use serde_json::json;

    use crate::meili_search::dump_response_status;
    use crate::movie::Movie;
    use crate::search::{MeiliSearchRequest, MeiliSearchResult, SearchPaginatedRequest};

    pub async fn meili_search_movie_paginated(
        entity: String,
        s: SearchPaginatedRequest,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        info!(
            "searching for entity {} and paginated request '{:?}'",
            &entity,
            s.clone()
        );

        let search_request = MeiliSearchRequest {
            q: s.q.clone(),
            offset: Some(s.offset),
            limit: Some(s.limit),
            page: None,
            hits_per_page: None,
            filter: None,
            facets: None,
            attributes_to_retrieve: None,
            attributes_to_crop: None,
            crop_marker: None,
            crop_length: None,
            attributes_to_highlight: None,
            highlight_pre_tag: None,
            highlight_post_tag: None,
            show_matches_position: None,
            sort: Some(s.sort.clone()),
            matching_strategy: None,
        };
        let json = json!(&search_request).to_string();
        info!(
            "search request for entity {} and paginated request '{:?}'   ->  \n{:?}\n    \n json: \n {} \n   ", &entity, &s, &search_request.clone(), &json
        );

        let index = format!("http://meilisearch01.bumzack.at/indexes/{}/search", &entity);

        let response = client
            .post(&index)
            .body(json.clone())
            .header("Authorization", "Bearer 1234567890123456".to_owned())
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        dump_response_status(&response, &index, &json);

        let movies = match response {
            Ok(r) => {
                let code = r.status();
                if code == StatusCode::OK
                    || code == StatusCode::ACCEPTED
                    || code == StatusCode::CREATED
                {
                    info!(
                        "meili_search_movie_paginated request success. unwrapping MeiliSearchResult<Movie>"
                    );
                    let result = r.json::<MeiliSearchResult<Movie>>().await;
                    match result {
                        Ok(r) => {
                            info!("meili_search_movie_paginated request success and all good. returning Vec<Movie>");
                            r.hits
                        }
                        Err(ee) => {
                            info!("meili_search_movie_paginated request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!(
                        "meili_search_movie_paginated request != OK. status {:?},     ",
                        code
                    );
                    error!(
                        "meili_search_movie_paginated request != OK. headers {:?},   ",
                        x
                    );
                    vec![]
                }
            }
            Err(eee) => {
                error!("meili_search_movie_paginated wtf?. error {:?}", eee);
                vec![]
            }
        };

        info!(
            "searching for entity {} and paginated request '{:?}'   ->  {} results ",
            &entity,
            &s,
            movies.len()
        );

        Ok(warp::reply::json(&movies))
    }
}

pub mod exec_meilisearch_search {
    use std::convert::Infallible;

    use reqwest::Client;
    use warp::hyper;

    use crate::meili_search::dump_response_status;

    pub async fn exec_meilisearch_search(
        entity_name: &String,
        json: String,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let index = format!(
            "http://meilisearch01.bumzack.at/indexes/{}/search",
            &entity_name
        );

        let response = client
            .post(&index)
            .body(json.clone())
            .header("Authorization", "Bearer 1234567890123456".to_owned())
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        dump_response_status(&response, &index, &json);

        // 🙏 https://github.com/seanmonstar/warp/issues/38

        // TODO unwrap no good here!
        let stream = response.unwrap().bytes_stream();
        let body = hyper::Body::wrap_stream(stream);
        Ok(warp::reply::Response::new(body))
    }
}

pub fn dump_response_status(response: &Result<Response, Error>, url: &String, json: &String) {
    match &response {
        Ok(res) => {
            let code = res.status();
            if code == StatusCode::OK || code == StatusCode::ACCEPTED || code == StatusCode::CREATED
            {
                info!("request success");
            } else {
                let x = res.headers().clone();
                error!("request != OK. status {:?},    url {}", code, url);
                error!("request != OK. headers {:?},    url {}", x, url);
                error!("remote address {:?}", res.remote_addr());
            }
        }
        Err(e) => error!(
            "request to meilisearch resulted in an error. request URL '{}', json '{}' error '{:?}'",
            url, json, e
        ),
    };
}
