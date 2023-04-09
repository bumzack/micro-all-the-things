pub mod meili_filter_movieaka {
    use std::collections::HashMap;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::entity::entity::Entity;
    use crate::meili::meili::mod_meili::meili_search;
    use crate::meili::meili_models::MeiliSearchResult;
    use crate::models::movieaka::MovieAkas;

    pub async fn meili_filter_movieaka_vec(
        filter_field: String,
        filter_values: Vec<String>,
        client: &Client,
    ) -> Vec<MovieAkas> {
        let mut filter = HashMap::new();
        filter.insert(filter_field, filter_values);

        let response = meili_search(
            Entity::MOVIEAKA,
            Some(filter),
            None,
            None,
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
                        "meili_filter_movieaka request success. unwrapping MeiliSearchResult<MovieAkas>"
                    );
                    let res = r.json::<MeiliSearchResult<MovieAkas>>().await;
                    match res {
                        Ok(r) => {
                            info!("meili_filter_movieaka request success and all good. returning Vec<MovieAkas>");
                            r.hits
                        }
                        Err(ee) => {
                            info!("meili_filter_movieaka request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!(
                        "meili_filter_movieaka request != OK. status {:?},     ",
                        code
                    );
                    error!("meili_filter_movieaka request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("meili_filter_movieaka wtf?. error {:?}", eee);
                vec![]
            }
        };
        movieakas
    }
}

pub mod meili_filter_person {
    use std::collections::HashMap;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::entity::entity::Entity;
    use crate::meili::meili::mod_meili::meili_search;
    use crate::meili::meili_models::MeiliSearchResult;
    use crate::models::person::Person;

    pub async fn meili_filter_person_vec(
        filter_field: String,
        filter_values: Vec<String>,
        client: &Client,
    ) -> Vec<Person> {
        let mut filter = HashMap::new();
        filter.insert(filter_field, filter_values);

        let response = meili_search(
            Entity::PERSON,
            Some(filter),
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
        persons
    }
}

pub mod meili_filter_principal {
    use std::collections::HashMap;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::entity::entity::Entity;
    use crate::meili::meili::mod_meili::meili_search;
    use crate::meili::meili_models::MeiliSearchResult;
    use crate::models::principal::Principal;

    pub async fn meili_filter_principal_vec(
        filter_field: String,
        filter_values: Vec<String>,
        client: &Client,
    ) -> Vec<Principal> {
        let mut filter = HashMap::new();
        filter.insert(filter_field, filter_values);

        let response = meili_search(
            Entity::PRINCIPAL,
            Some(filter),
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
        principals
    }
}

pub mod meili_filter_rating {
    use std::collections::HashMap;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::entity::entity::Entity;
    use crate::meili::meili::mod_meili::meili_search;
    use crate::meili::meili_models::MeiliSearchResult;
    use crate::models::rating::Rating;

    pub async fn meili_filter_rating_vec(
        filter_field: String,
        filter_values: Vec<String>,
        client: &Client,
    ) -> Vec<Rating> {
        let mut filter = HashMap::new();
        filter.insert(filter_field, filter_values);

        let response = meili_search(
            Entity::RATING,
            Some(filter),
            None,
            None,
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

        ratings
    }
}

pub mod meili_filter_crew {
    use std::collections::HashMap;

    use log::{error, info};
    use reqwest::{Client, StatusCode};

    use crate::entity::entity::Entity;
    use crate::meili::meili::mod_meili::meili_search;
    use crate::meili::meili_models::MeiliSearchResult;
    use crate::models::crew::Crew;

    pub async fn meili_filter_crew_vec(
        filter_field: String,
        filter_values: Vec<String>,
        client: &Client,
    ) -> Vec<Crew> {
        let mut filter = HashMap::new();
        filter.insert(filter_field, filter_values);

        let response = meili_search(
            Entity::CREW,
            Some(filter),
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
        crew
    }
}
