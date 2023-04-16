pub mod solr_entity_stuff {
    use std::collections::HashMap;

    use log::{error, info};
    use reqwest::{Client, StatusCode};
    use serde::{Deserialize, Serialize};

    use crate::entity::entity::Entity;
    use crate::solr::solr_http::mod_solr_http::solr_search_http;
    use crate::solr::solr_models::{FacetCounts, SolrResponse};

    pub async fn solr_filter_entity<T>(
        entity: Entity,
        filter_field: String,
        filter_values: Vec<String>,
        client: &Client,
    ) -> Vec<T>
    where
        T: for<'de> Deserialize<'de> + Serialize,
    {
        let mut filters = HashMap::new();
        filters.insert(filter_field, filter_values);

        let response =
            solr_search_http(entity, Some(filters), None, None, None, None, None, client);
        let response2 = response.await;
        let entities = match response2 {
            Ok(r) => {
                let code = r.status();
                if code == StatusCode::OK
                    || code == StatusCode::ACCEPTED
                    || code == StatusCode::CREATED
                {
                    let res = r.json::<SolrResponse<T>>().await;
                    match res {
                        Ok(r) => r.response.unwrap().docs.unwrap(),
                        Err(ee) => {
                            info!("solr_filter_person request error. returning empty Vec<T>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!(
                        "solr_filter request != OK. status {:?},     ",
                        code.as_u16()
                    );
                    error!("solr_filter request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!("solr_filter  wtf?. error {:?}", eee);
                vec![]
            }
        };
        entities
    }

    pub async fn solr_search_entity<T>(
        entity: Entity,
        search_text: String,
        _limit: u32,
        _offset: u32,
        facets: Vec<String>,
        client: &Client,
    ) -> Vec<T>
    where
        T: for<'de> Deserialize<'de> + Serialize,
    {
        let search_text_movie = vec![
            ("tconst".to_string(), search_text.clone()),
            ("titleType".to_string(), search_text.clone()),
            ("primaryTitle".to_string(), search_text.clone()),
            ("originalTitle".to_string(), search_text.clone()),
            ("adult".to_string(), search_text.clone()),
            ("genres".to_string(), search_text.clone()),
        ];

        let search_text_crew = vec![
            ("tconst".to_string(), search_text.clone()),
            ("directors".to_string(), search_text.clone()),
            ("writers".to_string(), search_text.clone()),
        ];

        let search_text_person = vec![
            ("nconst".to_string(), search_text.clone()),
            ("primary_name".to_string(), search_text.clone()),
            ("birthYear".to_string(), search_text.clone()),
            ("deathYear".to_string(), search_text.clone()),
            ("primaryProfession".to_string(), search_text.clone()),
            ("knownForTitles".to_string(), search_text.clone()),
        ];

        let search_text_principal = vec![
            ("tconst".to_string(), search_text.clone()),
            ("nconst".to_string(), search_text.clone()),
            ("category".to_string(), search_text.clone()),
            ("job".to_string(), search_text.clone()),
            ("characters".to_string(), search_text.clone()),
        ];

        let search_text_rating = vec![
            ("tconst".to_string(), search_text.clone()),
            ("average_rating".to_string(), search_text.clone()),
            ("num_votes".to_string(), search_text.clone()),
        ];

        let search_text_searchdoc = vec![
            ("tconst".to_string(), search_text.clone()),
            ("titles".to_string(), search_text.clone()),
            ("actors".to_string(), search_text.clone()),
            ("directors".to_string(), search_text.clone()),
            ("writers".to_string(), search_text.clone()),
            //     ("runtimeMinutes".to_string(), search_text.clone()),
            //      ("adult".to_string(), search_text.clone()),
            ("genres".to_string(), search_text.clone()),
            ("characters".to_string(), search_text.clone()),
            ("titleType".to_string(), search_text.clone()),
        ];

        let search_text_movieaka = vec![
            ("tconst".to_string(), search_text.clone()),
            ("region".to_string(), search_text.clone()),
            ("types".to_string(), search_text.clone()),
            ("language".to_string(), search_text.clone()),
            ("attributes".to_string(), search_text.clone()),
            ("originalTitle".to_string(), search_text.clone()),
            ("adult".to_string(), search_text.clone()),
            ("genres".to_string(), search_text.clone()),
            ("characters".to_string(), search_text.clone()),
            ("titleType".to_string(), search_text.clone()),
        ];

        let search_text_episode = vec![("tconst".to_string(), search_text.clone())];

        let search_text = match entity {
            Entity::MOVIE => search_text_movie,
            Entity::PERSON => search_text_person,
            Entity::CREW => search_text_crew,
            Entity::RATING => search_text_rating,
            Entity::EPISODE => search_text_episode,
            Entity::PRINCIPAL => search_text_principal,
            Entity::MOVIEAKA => search_text_movieaka,
            Entity::SEARCHINDEX => search_text_searchdoc,
        };

        let response = solr_search_http(
            entity,
            None,
            Some(facets),
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
                    let res = r.json::<SolrResponse<T>>().await;
                    match res {
                        Ok(r) => r.response.unwrap().docs.unwrap(),
                        Err(ee) => {
                            error!("solr_search_person request error. returning empty Vec<>. error {:?}",ee);
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

    pub async fn solr_search_entity_with_facets<T>(
        entity: Entity,
        search_text: String,
        _limit: u32,
        _offset: u32,
        facets: Vec<String>,
        client: &Client,
    ) -> (Vec<T>, Option<FacetCounts>)
    where
        T: for<'de> Deserialize<'de> + Serialize,
    {
        let search_text_movie = vec![
            ("tconst".to_string(), search_text.clone()),
            ("titleType".to_string(), search_text.clone()),
            ("primaryTitle".to_string(), search_text.clone()),
            ("originalTitle".to_string(), search_text.clone()),
            ("adult".to_string(), search_text.clone()),
            ("genres".to_string(), search_text.clone()),
        ];

        let search_text_crew = vec![
            ("tconst".to_string(), search_text.clone()),
            ("directors".to_string(), search_text.clone()),
            ("writers".to_string(), search_text.clone()),
        ];

        let search_text_person = vec![
            ("nconst".to_string(), search_text.clone()),
            ("primary_name".to_string(), search_text.clone()),
            ("birthYear".to_string(), search_text.clone()),
            ("deathYear".to_string(), search_text.clone()),
            ("primaryProfession".to_string(), search_text.clone()),
            ("knownForTitles".to_string(), search_text.clone()),
        ];

        let search_text_principal = vec![
            ("tconst".to_string(), search_text.clone()),
            ("nconst".to_string(), search_text.clone()),
            ("category".to_string(), search_text.clone()),
            ("job".to_string(), search_text.clone()),
            ("characters".to_string(), search_text.clone()),
        ];

        let search_text_rating = vec![
            ("tconst".to_string(), search_text.clone()),
            ("average_rating".to_string(), search_text.clone()),
            ("num_votes".to_string(), search_text.clone()),
        ];

        let search_text_searchdoc = vec![
            ("tconst".to_string(), search_text.clone()),
            ("titles".to_string(), search_text.clone()),
            ("actors".to_string(), search_text.clone()),
            ("directors".to_string(), search_text.clone()),
            ("writers".to_string(), search_text.clone()),
            //     ("runtimeMinutes".to_string(), search_text.clone()),
            //      ("adult".to_string(), search_text.clone()),
            ("genres".to_string(), search_text.clone()),
            ("characters".to_string(), search_text.clone()),
            ("titleType".to_string(), search_text.clone()),
        ];

        let search_text_movieaka = vec![
            ("tconst".to_string(), search_text.clone()),
            ("region".to_string(), search_text.clone()),
            ("types".to_string(), search_text.clone()),
            ("language".to_string(), search_text.clone()),
            ("attributes".to_string(), search_text.clone()),
            ("originalTitle".to_string(), search_text.clone()),
            ("adult".to_string(), search_text.clone()),
            ("genres".to_string(), search_text.clone()),
            ("characters".to_string(), search_text.clone()),
            ("titleType".to_string(), search_text.clone()),
        ];

        let search_text_episode = vec![("tconst".to_string(), search_text.clone())];

        let search_text = match entity {
            Entity::MOVIE => search_text_movie,
            Entity::PERSON => search_text_person,
            Entity::CREW => search_text_crew,
            Entity::RATING => search_text_rating,
            Entity::EPISODE => search_text_episode,
            Entity::PRINCIPAL => search_text_principal,
            Entity::MOVIEAKA => search_text_movieaka,
            Entity::SEARCHINDEX => search_text_searchdoc,
        };

        let response = solr_search_http(
            entity,
            None,
            Some(facets),
            Some(search_text),
            None,
            None,
            None,
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
                    let res = r.json::<SolrResponse<T>>().await;
                    match res {
                        Ok(r) => {
                            let re = r.response.unwrap();
                            // let facets = r.facet_counts;
                            let docs = re.docs.unwrap();
                            //  (docs, facets)
                            (docs, None)
                        }
                        Err(ee) => {
                            error!("solr_search_person request error. returning empty Vec<>. error {:?}",ee);
                            (vec![], None)
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!("solr_search_person request != OK. status {:?},     ", code);
                    error!("solr_search_person request != OK. headers {:?},   ", x);
                    (vec![], None)
                }
            }
            Err(eee) => {
                error!("solr_search_person wtf?. error {:?}", eee);
                (vec![], None)
            }
        };
        docs
    }

    pub async fn solr_read_doc<T>(
        entity: Entity,
        offset: u32,
        limit: u32,
        client: &Client,
    ) -> Vec<T>
    where
        T: for<'de> Deserialize<'de> + Serialize,
    {
        let sort = vec![("id".to_string(), true)];
        let response = solr_search_http(
            entity,
            None,
            None,
            None,
            Some(sort),
            Some(limit),
            Some(offset),
            client,
        );

        let response2 = response.await;
        if response2.is_err() {
            error!("error requesting solr index  {}", response2.err().unwrap());
            return vec![];
        }
        let result = response2.unwrap().json::<SolrResponse<T>>().await;
        if result.is_err() {
            error!(
                "can't unwrap response to SolrResponse<T> type. error {}",
                result.err().unwrap()
            );
            return vec![];
        }
        let result = result.unwrap();

        match result.response {
            Some(m) => m.docs.unwrap(),
            None => vec![],
        }
    }
}
