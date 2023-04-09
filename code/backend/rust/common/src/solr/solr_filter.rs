pub mod solr_filter_person {
    use std::collections::HashMap;

    use log::{error, info};
    use reqwest::{Client, StatusCode};
    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Serialize};

    use crate::entity::entity::Entity;
    use crate::models::person::Person;
    use crate::solr::models::{SolrResponse, SolrResponse2};
    use crate::solr::solr::mod_solr::solr_search;

    pub async fn solr_filter_person_vec(
        filter_field: String,
        filter_values: Vec<String>,
        client: &Client,
    ) -> Vec<Person> {
        let mut filters = HashMap::new();
        filters.insert(filter_field, filter_values);

        let response = solr_search(
            Entity::PERSON,
            Some(filters),
            None,
            None,
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
                    info!("solr_filter_person request success. unwrapping SolrResponse<Person>");
                    let res = r.json::<SolrResponse<Person>>().await;
                    match res {
                        Ok(r) => {
                            info!("solr_filter_person request success and all good. returning Vec<Person>");
                            r.response.unwrap().docs.unwrap()
                        }
                        Err(ee) => {
                            info!("solr_filter_person request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!("solr_filter_person request != OK. status {:?},     ", code);
                    error!("solr_filter_person request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("solr_filter_person wtf?. error {:?}", eee);
                vec![]
            }
        };
        persons
    }

    pub async fn solr_filter_generic_vec<'a, T>(
        filter_field: String,
        filter_values: Vec<String>,
        client: &Client,
    ) -> Vec<T>
    where
        T: for<'de> Deserialize<'de> + Serialize,
    {
        let mut filters = HashMap::new();
        filters.insert(filter_field, filter_values);

        let response = solr_search(
            Entity::PERSON,
            Some(filters),
            None,
            None,
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
                    info!("solr_filter_person request success. unwrapping SolrResponse<Person>");
                    let res = r.json::<SolrResponse2<T>>().await;
                    match res {
                        Ok(r) => {
                            info!("solr_filter_person request success and all good. returning Vec<Person>");
                            r.response.unwrap().docs.unwrap()
                        }
                        Err(ee) => {
                            info!("solr_filter_person request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!("solr_filter_person request != OK. status {:?},     ", code);
                    error!("solr_filter_person request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("solr_filter_person wtf?. error {:?}", eee);
                vec![]
            }
        };
        persons
    }
}

pub mod solr_filter_principal {
    use std::collections::HashMap;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::entity::entity::Entity;
    use crate::models::principal::Principal;
    use crate::solr::models::SolrResponse;
    use crate::solr::solr::mod_solr::solr_search;

    pub async fn solr_filter_principal_vec(
        filter_field: String,
        filter_values: Vec<String>,
        client: &Client,
    ) -> Vec<Principal> {
        let mut filters = HashMap::new();
        filters.insert(filter_field, filter_values);

        let response = solr_search(
            Entity::PRINCIPAL,
            Some(filters),
            None,
            None,
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
                    info!(
                        "solr_filter_principal request success. unwrapping SolrResponse<Principal>"
                    );
                    let res = r.json::<SolrResponse<Principal>>().await;
                    match res {
                        Ok(r) => {
                            info!("solr_filter_principal request success and all good. returning Vec<Principal>");
                            r.response.unwrap().docs.unwrap()
                        }
                        Err(ee) => {
                            info!("solr_filter_principal request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!(
                        "solr_filter_principal request != OK. status {:?},     ",
                        code
                    );
                    error!("solr_filter_principal request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("solr_filter_principal wtf?. error {:?}", eee);
                vec![]
            }
        };
        principals
    }
}

pub mod solr_filter_crew {
    use std::collections::HashMap;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::entity::entity::Entity;
    use crate::models::crew::Crew;
    use crate::solr::models::SolrResponse;
    use crate::solr::solr::mod_solr::solr_search;

    pub async fn solr_filter_crew_vec(
        filter_field: String,
        filter_values: Vec<String>,
        client: &Client,
    ) -> Vec<Crew> {
        let mut filters = HashMap::new();
        filters.insert(filter_field, filter_values);

        let response = solr_search(
            Entity::CREW,
            Some(filters),
            None,
            None,
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
                    info!("solr_filter_crew request success. unwrapping SolrResponse<Crew>");
                    let res = r.json::<SolrResponse<Crew>>().await;
                    match res {
                        Ok(r) => {
                            info!("solr_filter_crew request success and all good. returning Vec<Principal>");
                            r.response.unwrap().docs.unwrap()
                        }
                        Err(ee) => {
                            info!(
                                "solr_filter_crew request error. returning empty Vec<>. error {:?}",
                                ee
                            );
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!("solr_filter_crew request != OK. status {:?},     ", code);
                    error!("solr_filter_crew request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("solr_filter_crew wtf?. error {:?}", eee);
                vec![]
            }
        };
        crew
    }
}

pub mod solr_filter_rating {
    use std::collections::HashMap;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::entity::entity::Entity;
    use crate::models::rating::Rating;
    use crate::solr::models::SolrResponse;
    use crate::solr::solr::mod_solr::solr_search;

    pub async fn solr_filter_rating_vec(
        filter_field: String,
        filter_values: Vec<String>,
        client: &Client,
    ) -> Vec<Rating> {
        let mut filters = HashMap::new();
        filters.insert(filter_field, filter_values);

        let response = solr_search(
            Entity::RATING,
            Some(filters),
            None,
            None,
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
                    info!("solr_filter_rating request success. unwrapping SolrResponse<Rating>");
                    let res = r.json::<SolrResponse<Rating>>().await;
                    match res {
                        Ok(r) => {
                            info!("solr_filter_rating request success and all good. returning Vec<Rating>");
                            r.response.unwrap().docs.unwrap()
                        }
                        Err(ee) => {
                            info!("solr_filter_rating request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!("solr_filter_rating request != OK. status {:?},     ", code);
                    error!("solr_filter_rating request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("solr_filter_rating wtf?. error {:?}", eee);
                vec![]
            }
        };
        crew
    }
}

pub mod solr_filter_movieaka {
    use std::collections::HashMap;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::entity::entity::Entity;
    use crate::models::movieaka::MovieAkas;
    use crate::solr::models::SolrResponse;
    use crate::solr::solr::mod_solr::solr_search;

    pub async fn solr_filter_movieaka_vec(
        filter_field: String,
        filter_values: Vec<String>,
        client: &Client,
    ) -> Vec<MovieAkas> {
        let mut filters = HashMap::new();
        filters.insert(filter_field, filter_values);

        let response = solr_search(
            Entity::MOVIEAKA,
            Some(filters),
            None,
            None,
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
                    info!(
                        "solr_filter_movieaka request success. unwrapping SolrResponse<MovieAkas>"
                    );
                    let res = r.json::<SolrResponse<MovieAkas>>().await;
                    match res {
                        Ok(r) => {
                            info!("solr_filter_movieaka request success and all good. returning Vec<Rating>");
                            r.response.unwrap().docs.unwrap()
                        }
                        Err(ee) => {
                            info!("solr_filter_movieaka request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!(
                        "solr_filter_movieaka request != OK. status {:?},     ",
                        code
                    );
                    error!("solr_filter_movieaka request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("solr_filter_movieaka wtf?. error {:?}", eee);
                vec![]
            }
        };
        crew
    }
}
