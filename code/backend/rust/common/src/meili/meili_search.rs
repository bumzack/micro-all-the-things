pub mod meili_search_searchindex {
    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::entity::entity::Entity;
    use crate::meili::meili::mod_meili::meili_search;
    use crate::meili::meili_models::MeiliSearchResult;
    use crate::models::search_doc::SearchIndexDoc;

    pub async fn meili_search_searchindex_vec(
        search_text: String,
        limit: u32,
        offset: u32,
        facets: Vec<String>,
        client: &Client,
    ) -> Vec<SearchIndexDoc> {
        let search_text = vec![("ignored for meili".to_string(), search_text)];
        let response = meili_search(
            Entity::SEARCHINDEX,
            None,
            Some(facets),
            Some(search_text),
            None,
            Some(limit),
            Some(offset),
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
        docs
    }
}

pub mod meili_search_person {
    use std::convert::Infallible;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::entity::entity::Entity;
    use crate::meili::meili::mod_meili::meili_search;
    use crate::meili::meili_models::MeiliSearchResult;
    use crate::models::person::Person;

    pub async fn meili_search_person(
        search_text: String,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let search_text = vec![("ignored for meili".to_string(), search_text)];
        let response = meili_search(
            Entity::PERSON,
            None,
            None,
            Some(search_text),
            None,
            None,
            None,
            client,
        );

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
    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::entity::entity::Entity;
    use crate::logging::logging_service_client::logging_service;
    use crate::meili::meili::mod_meili::meili_search;
    use crate::meili::meili_models::MeiliSearchResult;
    use crate::models::movie::Movie;

    pub async fn meili_search_movie_vec(search_text: String, client: &Client) -> Vec<Movie> {
        let search_text = vec![("ignored for meili".to_string(), search_text)];
        let response = meili_search(
            Entity::MOVIE,
            None,
            None,
            Some(search_text.clone()),
            None,
            None,
            None,
            client,
        );

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
            "finished meili_search_movie(). search_text '{:?}' returned {} movies ",
            search_text.clone(),
            movies.len()
        );
        logging_service::log_entry(
            "rust_create_search_index".to_string(),
            "INFO".to_string(),
            &msg,
        )
            .await;
        movies
    }
}

pub mod meili_search_movieaka {
    use std::convert::Infallible;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::entity::entity::Entity;
    use crate::meili::meili::mod_meili::meili_search;
    use crate::meili::meili_models::MeiliSearchResult;
    use crate::models::movieaka::MovieAkas;

    pub async fn meili_search_movieaka(
        search_text: String,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let search_text = vec![("ignored for meili".to_string(), search_text)];
        let response = meili_search(
            Entity::MOVIEAKA,
            None,
            None,
            Some(search_text),
            None,
            None,
            None,
            client,
        );

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
                error!("meili_search_movieakas wtf?. error {:?}", eee);
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

    use crate::entity::entity::Entity;
    use crate::meili::meili::mod_meili::meili_search;
    use crate::meili::meili_models::MeiliSearchResult;
    use crate::models::crew::Crew;

    pub async fn meili_search_crew(
        search_text: String,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let search_text = vec![("ignored for meili".to_string(), search_text)];
        let response = meili_search(
            Entity::CREW,
            None,
            None,
            Some(search_text),
            None,
            None,
            None,
            client,
        );

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

    use crate::entity::entity::Entity;
    use crate::meili::meili::mod_meili::meili_search;
    use crate::meili::meili_models::MeiliSearchResult;
    use crate::models::rating::Rating;

    pub async fn meili_search_episode(
        search_text: String,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let search_text = vec![("ignored for meili".to_string(), search_text)];
        let response = meili_search(
            Entity::EPISODE,
            None,
            None,
            Some(search_text),
            None,
            None,
            None,
            client,
        );

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
