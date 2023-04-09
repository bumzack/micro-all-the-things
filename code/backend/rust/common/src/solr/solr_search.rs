pub mod solr_search_person {
    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::entity::entity::Entity;
    use crate::models::person::Person;
    use crate::solr::models::SolrResponse;
    use crate::solr::solr::mod_solr::solr_search;

    async fn solr_search_person_vec(search_text: String, client: &Client) -> Vec<Person> {
        let search_text = vec![
            ("nconst".to_string(), search_text.clone()),
            ("primary_name".to_string(), search_text.clone()),
            ("birth_year".to_string(), search_text.clone()),
            ("death_year".to_string(), search_text.clone()),
            ("primary_profession".to_string(), search_text.clone()),
            ("known_for_titles".to_string(), search_text.clone()),
        ];

        let response = solr_search(
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
                    info!("solr_search_person request success. unwrapping  SolrResponse<Person>");
                    let res = r.json::<SolrResponse<Person>>().await;
                    match res {
                        Ok(r) => {
                            info!("solr_search_person request success and all good. returning Vec<Person>");
                            r.response.unwrap().docs.unwrap()
                        }
                        Err(ee) => {
                            info!("solr_search_person request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!("solr_search_person request != OK. status {:?},     ", code);
                    error!("solr_search_person request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("solr_search_person wtf?. error {:?}", eee);
                vec![]
            }
        };
        persons
    }
}

pub mod solr_search_principal {
    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::entity::entity::Entity;
    use crate::models::principal::Principal;
    use crate::solr::models::SolrResponse;
    use crate::solr::solr::mod_solr::solr_search;

    pub async fn solr_search_principal_vec(search_text: String, client: &Client) -> Vec<Principal> {
        let search_text = vec![
            ("tconst".to_string(), search_text.clone()),
            ("nconst".to_string(), search_text.clone()),
            ("category".to_string(), search_text.clone()),
            ("job".to_string(), search_text.clone()),
            ("characters".to_string(), search_text.clone()),
        ];

        let response = solr_search(
            Entity::PRINCIPAL,
            None,
            None,
            Some(search_text),
            None,
            None,
            None,
            client,
        );

        let response2 = response.await;
        let principals = match response2 {
            Ok(r) => {
                let code = r.status();
                if code == StatusCode::OK
                    || code == StatusCode::ACCEPTED
                    || code == StatusCode::CREATED
                {
                    info!("solr_search_principal request success. unwrapping  SolrResponse<Principal>");
                    let res = r.json::<SolrResponse<Principal>>().await;
                    match res {
                        Ok(r) => {
                            info!("solr_search_principal request success and all good. returning Vec<Principal>");
                            r.response.unwrap().docs.unwrap()
                        }
                        Err(ee) => {
                            info!("solr_search_principal request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!(
                        "solr_search_principal request != OK. status {:?},     ",
                        code
                    );
                    error!("solr_search_principal request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("solr_search_principal wtf?. error {:?}", eee);
                vec![]
            }
        };
        principals
    }
}

pub mod solr_search_movie {
    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::entity::entity::Entity;
    use crate::models::movie::Movie;
    use crate::solr::models::SolrResponse;
    use crate::solr::solr::mod_solr::solr_search;

    pub async fn solr_search_movie_vec(search_text: String, client: &Client) -> Vec<Movie> {
        let search_text = vec![
            ("tconst".to_string(), search_text.clone()),
            ("title_type".to_string(), search_text.clone()),
            ("primary_title".to_string(), search_text.clone()),
            ("original_title".to_string(), search_text.clone()),
            ("characters".to_string(), search_text.clone()),
            ("adult".to_string(), search_text.clone()),
            ("start_year".to_string(), search_text.clone()),
            ("end_year".to_string(), search_text.clone()),
            ("runtime_minutes".to_string(), search_text.clone()),
            ("genres".to_string(), search_text.clone()),
        ];

        let response = solr_search(
            Entity::PRINCIPAL,
            None,
            None,
            Some(search_text),
            None,
            None,
            None,
            client,
        );

        let response2 = response.await;
        let principals = match response2 {
            Ok(r) => {
                let code = r.status();
                if code == StatusCode::OK
                    || code == StatusCode::ACCEPTED
                    || code == StatusCode::CREATED
                {
                    info!("solr_search_principal request success. unwrapping  SolrResponse<Principal>");
                    let res = r.json::<SolrResponse<Movie>>().await;
                    match res {
                        Ok(r) => {
                            info!("solr_search_principal request success and all good. returning Vec<Principal>");
                            r.response.unwrap().docs.unwrap()
                        }
                        Err(ee) => {
                            info!("solr_search_principal request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!(
                        "solr_search_principal request != OK. status {:?},     ",
                        code
                    );
                    error!("solr_search_principal request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("solr_search_principal wtf?. error {:?}", eee);
                vec![]
            }
        };
        principals
    }
}

pub mod solr_search_rating {
    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::entity::entity::Entity;
    use crate::models::rating::Rating;
    use crate::solr::models::SolrResponse;
    use crate::solr::solr::mod_solr::solr_search;

    async fn solr_search_rating_vec(search_text: String, client: &Client) -> Vec<Rating> {
        let search_text = vec![
            ("tconst".to_string(), search_text.clone()),
            ("average_rating".to_string(), search_text.clone()),
            ("num_votes".to_string(), search_text.clone()),
        ];

        let response = solr_search(
            Entity::PRINCIPAL,
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
                    info!("solr_search_rating  request success. unwrapping  SolrResponse<Rating>");
                    let res = r.json::<SolrResponse<Rating>>().await;
                    match res {
                        Ok(r) => {
                            info!("solr_search_rating request success and all good. returning Vec<Rating>");
                            r.response.unwrap().docs.unwrap()
                        }
                        Err(ee) => {
                            info!("solr_search_rating request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!("solr_search_rating request != OK. status {:?},    ", code);
                    error!("solr_search_rating request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("solr_search_rating wtf?. error {:?}", eee);
                vec![]
            }
        };
        ratings
    }
}

pub mod solr_search_crew {
    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::entity::entity::Entity;
    use crate::models::crew::Crew;
    use crate::solr::models::SolrResponse;
    use crate::solr::solr::mod_solr::solr_search;

    async fn solr_search_crew_vec(search_text: String, client: &Client) -> Vec<Crew> {
        let search_text = vec![
            ("tconst".to_string(), search_text.clone()),
            ("directors".to_string(), search_text.clone()),
            ("writers".to_string(), search_text.clone()),
        ];

        let response = solr_search(
            Entity::PRINCIPAL,
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
                    info!("solr_search_crew request success. unwrapping  SolrResponse<Crew>");
                    let res = r.json::<SolrResponse<Crew>>().await;
                    match res {
                        Ok(r) => {
                            info!("solr_search_crew request success and all good. returning Vec<Crew>");
                            r.response.unwrap().docs.unwrap()
                        }
                        Err(ee) => {
                            info!(
                                "solr_search_crew request error. returning empty Vec<>. error {:?}",
                                ee
                            );
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!("solr_search_crew request != OK. status {:?},     ", code);
                    error!("solr_search_crew request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("solr_search_crew wtf?. error {:?}", eee);
                vec![]
            }
        };
        crew
    }
}

pub mod solr_search_search_index {
    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::entity::entity::Entity;
    use crate::models::search_doc::SearchIndexDoc;
    use crate::solr::models::SolrResponse;
    use crate::solr::solr::mod_solr::solr_search;

    pub async fn solr_search_search_index_vec(
        search_text: String,
        limit: u32,
        offset: u32,
        facets: Vec<String>,
        client: &Client,
    ) -> Vec<SearchIndexDoc> {
        let search_text = vec![
            ("tconst".to_string(), search_text.clone()),
            ("titles".to_string(), search_text.clone()),
            ("actors".to_string(), search_text.clone()),
            ("directors".to_string(), search_text.clone()),
            ("writers".to_string(), search_text.clone()),
            ("runtime_minutes".to_string(), search_text.clone()),
            ("adult".to_string(), search_text.clone()),
            ("genres".to_string(), search_text.clone()),
            ("characters".to_string(), search_text.clone()),
        ];

        let response = solr_search(
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
        let search_index_docs = match response2 {
            Ok(r) => {
                let code = r.status();
                if code == StatusCode::OK
                    || code == StatusCode::ACCEPTED
                    || code == StatusCode::CREATED
                {
                    info!("solr_search_search_index_doc request success. unwrapping  SolrResponse<SearchIndexDoc>");
                    let res = r.json::<SolrResponse<SearchIndexDoc>>().await;
                    match res {
                        Ok(r) => {
                            info!("solr_search_search_index_doc request success and all good. returning Vec<SearchIndexDoc>");
                            r.response.unwrap().docs.unwrap()
                        }
                        Err(ee) => {
                            info!(
                                "solr_search_search_index_doc request error. returning empty Vec<>. error {:?}",
                                ee
                            );
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!(
                        "solr_search_search_index_doc request != OK. status {:?},     ",
                        code
                    );
                    error!(
                        "solr_search_search_index_doc request != OK. headers {:?},   ",
                        x
                    );
                    vec![]
                }
            }
            Err(eee) => {
                error!("solr_search_search_index_doc wtf?. error {:?}", eee);
                vec![]
            }
        };
        search_index_docs
    }
}
