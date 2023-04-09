pub mod solr_entity_stuff {
    use std::collections::HashMap;

    use log::{error, info};
    use reqwest::{Client, StatusCode};
    use serde::{Deserialize, Serialize};

    use crate::entity::entity::Entity;
    use crate::solr::models::SolrResponse;
    use crate::solr::solr_http::mod_solr_http::solr_search_http;

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
                    info!("solr_filter  request success. unwrapping SolrResponse<T>");

                    let res = r.json::<SolrResponse<T>>().await;
                    match res {
                        Ok(r) => {
                            info!(
                                "solr_filter_person request success and all good. returning Vec<T>"
                            );
                            r.response.unwrap().docs.unwrap()
                        }
                        Err(ee) => {
                            info!("solr_filter_person request error. returning empty Vec<T>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!("solr_filter request != OK. status {:?},     ", code);
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
        _facets: Vec<String>,
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
            ("birth_year".to_string(), search_text.clone()),
            ("death_year".to_string(), search_text.clone()),
            ("primary_profession".to_string(), search_text.clone()),
            ("known_for_titles".to_string(), search_text.clone()),
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
            ("runtime_minutes".to_string(), search_text.clone()),
            ("adult".to_string(), search_text.clone()),
            ("genres".to_string(), search_text.clone()),
            ("characters".to_string(), search_text.clone()),
            ("title_type".to_string(), search_text.clone()),
        ];

        let search_text_movieaka = vec![
            ("tconst".to_string(), search_text.clone()),
            ("region".to_string(), search_text.clone()),
            ("types".to_string(), search_text.clone()),
            ("language".to_string(), search_text.clone()),
            ("attributes".to_string(), search_text.clone()),
            ("original_title".to_string(), search_text.clone()),
            ("adult".to_string(), search_text.clone()),
            ("genres".to_string(), search_text.clone()),
            ("characters".to_string(), search_text.clone()),
            ("title_type".to_string(), search_text.clone()),
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
                    info!("solr_search_person request success. unwrapping  SolrResponse<T>");
                    let res = r.json::<SolrResponse<T>>().await;
                    match res {
                        Ok(r) => {
                            info!(
                                "solr_search_person request success and all good. returning Vec<T>"
                            );
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
                "cant unwrap response to SolrResponse<T> type. error {}",
                result.err().unwrap()
            );
            return vec![];
        }
        let result = result.unwrap();

        let entities = match result.response {
            Some(m) => m.docs.unwrap(),
            None => vec![],
        };
        entities
    }
}
