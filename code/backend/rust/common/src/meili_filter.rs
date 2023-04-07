pub mod meili_filter_person {
    use std::convert::Infallible;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::meili_filter::meili_filter::meili_filter;
    use crate::person::Person;
    use crate::search::MeiliSearchResult;

    pub async fn meili_filter_person(
        entity: String,
        filter: Vec<String>,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_filter(entity, filter, client);

        let response2 = response.await;
        let persons = match response2 {
            Ok(r) => {
                let code = r.status();
                if code == StatusCode::OK
                    || code == StatusCode::ACCEPTED
                    || code == StatusCode::CREATED
                {
                    info!(
                        "meili_filter_person request success. unwrapping MeiliSearchResult<Person>"
                    );
                    let res = r.json::<MeiliSearchResult<Person>>().await;
                    match res {
                        Ok(r) => {
                            info!("meili_filter_person request success and all good. returning Vec<Person>");
                            r.hits
                        }
                        Err(ee) => {
                            info!("meili_filter_person request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!("meili_filter_person request != OK. status {:?},     ", code);
                    error!("meili_filter_person request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("meili_filter_person wtf?. error {:?}", eee);
                vec![]
            }
        };

        Ok(warp::reply::json(&persons))
    }
}

pub mod meili_filter_principal {
    use std::convert::Infallible;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::meili_filter::meili_filter::meili_filter;
    use crate::principal::Principal;
    use crate::search::MeiliSearchResult;

    pub async fn meili_filter_principal(
        entity: String,
        filter: Vec<String>,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_filter(entity, filter, client);

        let response2 = response.await;
        let principals = match response2 {
            Ok(r) => {
                let code = r.status();
                if code == StatusCode::OK
                    || code == StatusCode::ACCEPTED
                    || code == StatusCode::CREATED
                {
                    info!("meili_filter_principal request success. unwrapping MeiliSearchResult<Principal>");
                    let res = r.json::<MeiliSearchResult<Principal>>().await;
                    match res {
                        Ok(r) => {
                            info!("meili_filter_principal request success and all good. returning Vec<Principal>");
                            r.hits
                        }
                        Err(ee) => {
                            info!("meili_filter_principal request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!(
                        "meili_filter_principal request != OK. status {:?},     ",
                        code
                    );
                    error!("meili_filter_principal request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("meili_filter_principal wtf?. error {:?}", eee);
                vec![]
            }
        };

        Ok(warp::reply::json(&principals))
    }
}

pub mod meili_filter_rating {
    use std::convert::Infallible;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::meili_filter::meili_filter::meili_filter;
    use crate::rating::Rating;
    use crate::search::MeiliSearchResult;

    pub async fn meili_filter_rating(
        entity: String,
        filter: Vec<String>,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_filter(entity, filter, client);

        let response2 = response.await;
        let principals = match response2 {
            Ok(r) => {
                let code = r.status();
                if code == StatusCode::OK
                    || code == StatusCode::ACCEPTED
                    || code == StatusCode::CREATED
                {
                    info!("meili_filter_rating  request success. unwrapping MeiliSearchResult<Rating>");
                    let res = r.json::<MeiliSearchResult<Rating>>().await;
                    match res {
                        Ok(r) => {
                            info!("meili_filter_rating request success and all good. returning Vec<Rating>");
                            r.hits
                        }
                        Err(ee) => {
                            info!("meili_filter_rating request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!("meili_filter_rating request != OK. status {:?},    ", code);
                    error!("meili_filter_rating request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("meili_filter_rating wtf?. error {:?}", eee);
                vec![]
            }
        };

        Ok(warp::reply::json(&principals))
    }
}

pub mod meili_filter_crew {
    use std::convert::Infallible;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::crew::Crew;
    use crate::meili_filter::meili_filter::meili_filter;
    use crate::search::MeiliSearchResult;

    pub async fn meili_filter_crew(
        entity: String,
        filter: Vec<String>,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_filter(entity, filter, client);

        let response2 = response.await;
        let crew = match response2 {
            Ok(r) => {
                let code = r.status();
                if code == StatusCode::OK
                    || code == StatusCode::ACCEPTED
                    || code == StatusCode::CREATED
                {
                    info!("meili_filter_crew request success. unwrapping MeiliSearchResult<Crew>");
                    let res = r.json::<MeiliSearchResult<Crew>>().await;
                    match res {
                        Ok(r) => {
                            info!("meili_filter_crew request success and all good. returning Vec<Crew>");
                            r.hits
                        }
                        Err(ee) => {
                            info!("meili_filter_crew request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!("meili_filter_crew request != OK. status {:?},     ", code);
                    error!("meili_filter_crew request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("meili_filter_crew wtf?. error {:?}", eee);
                vec![]
            }
        };

        Ok(warp::reply::json(&crew))
    }
}

pub mod meili_filter {
    use log::info;
    use reqwest::{Client, Error, Response};
    use serde_json::json;

    use crate::meili_search::dump_response_status;
    use crate::search::MeiliSearchRequest;

    pub(crate) async fn meili_filter(
        entity: String,
        filter: Vec<String>,
        client: &Client,
    ) -> Result<Response, Error> {
        let search_request = MeiliSearchRequest {
            q: "*".to_string(),
            offset: None,
            limit: None,
            page: None,
            hits_per_page: None,
            filter: Some(filter.clone()),
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
            "meili_filter entity {}, filter {:?}, request_json {}",
            &entity, &filter, &json
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
