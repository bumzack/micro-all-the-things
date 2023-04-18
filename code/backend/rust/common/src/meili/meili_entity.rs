pub mod meili_entity_stuff {
    use std::collections::HashMap;

    use log::{error, info};
    use reqwest::{Client, StatusCode};
    use serde::{Deserialize, Serialize};

    use crate::entity::entity::Entity;
    use crate::meili::meili_http::meili_http_stuff::{meili_read_document_http, meili_search_http};
    use crate::meili::meili_models::MeiliSearchResult;
    use crate::models::search_doc::IndexDocFacetDistribution;

    pub async fn meili_filter_entity<T>(
        entity: Entity,
        filter_field: String,
        filter_values: Vec<String>,
        client: &Client,
    ) -> Vec<T>
        where
            T: for<'de> Deserialize<'de> + Serialize,
    {
        let mut filter = HashMap::new();
        filter.insert(filter_field, filter_values);

        let response =
            meili_search_http(entity, Some(filter), None, None, None, None, None, client);

        let response2 = response.await;
        let entities = match response2 {
            Ok(r) => {
                let code = r.status();
                if code == StatusCode::OK
                    || code == StatusCode::ACCEPTED
                    || code == StatusCode::CREATED
                {
                    let res = r.json::<MeiliSearchResult<T>>().await;
                    match res {
                        Ok(r) => r.hits,
                        Err(ee) => {
                            error!(" meili_filter_entity request error. returning empty Vec<>. error {:?}",ee);
                            vec![]
                        }
                    }
                } else {
                    let x = r.headers().clone();
                    error!(
                        " meili_filter_entity request != OK. status {:?},     ",
                        code
                    );
                    error!(" meili_filter_entity request != OK. headers {:?},   ", x);
                    vec![]
                }
            }
            Err(eee) => {
                error!(" meili_filter_entity wtf?. error {:?}", eee);
                vec![]
            }
        };
        entities
    }

    pub async fn meili_search_entity<T>(
        entity: Entity,
        search_text: String,
        limit: u32,
        offset: u32,
        facets: Vec<String>,
        client: &Client,
    ) -> Vec<T>
        where
            T: for<'de> Deserialize<'de> + Serialize,
    {
        let search_text = vec![("ignored for meili".to_string(), search_text)];
        let response = meili_search_http(
            entity,
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
                    let result = r.json::<MeiliSearchResult<T>>().await;
                    match result {
                        Ok(r) => r.hits,
                        Err(ee) => {
                            error!("meili_search_searchindex request error. returning empty Vec<>. error {:?}",ee);
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

    pub async fn meili_search_entity_with_facets<T>(
        entity: Entity,
        search_text: String,
        limit: u32,
        offset: u32,
        facets: Vec<String>,
        client: &Client,
    ) -> (Vec<T>, Option<IndexDocFacetDistribution>)
        where
            T: for<'de> Deserialize<'de> + Serialize,
    {
        info!(
            "meili_search_entity_with_facets search_text {}, limit  {:?}, offset {:?}, facets: {:?}",
            &search_text,  limit, offset, facets
        );

        let search_text = vec![("ignored for meili".to_string(), search_text)];

        let response = meili_search_http(
            entity,
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
                    error!("meili_search_searchindex got a OK; ACCEPTED or CREATE response");
                    let result = r.json::<MeiliSearchResult<T>>().await;
                    match result {
                        Ok(r) => {
                            let hits = r.hits;
                            let facets = r.facet_distribution;
                            (hits, facets)
                        }
                        Err(ee) => {
                            error!("meili_search_searchindex request error. returning empty Vec<>. error {:?}",ee);
                            (vec![], None)
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
                    (vec![], None)
                }
            }
            Err(eee) => {
                error!("meili_search_searchindex wtf?. error {:?}", eee);
                (vec![], None)
            }
        };
        docs
    }

    pub async fn meili_read_doc<T>(
        entity: Entity,
        offset: u32,
        limit: u32,
        client: &Client,
    ) -> Vec<T>
        where
            T: for<'de> Deserialize<'de> + Serialize,
    {
        let _sort = vec![("id".to_string(), true)];
        let response = meili_read_document_http(entity, offset, limit, client);

        let response2 = response.await;
        if response2.is_err() {
            error!(
                "error requesting meilisearch index  {}",
                response2.err().unwrap()
            );
            return vec![];
        }
        let result = response2.unwrap().json::<MeiliSearchResult<T>>().await;
        if result.is_err() {
            error!(
                "cant unwrap response to MeiliSearchResult<T> type. error {}",
                result.err().unwrap()
            );
            return vec![];
        }

        match result {
            Ok(meilisearch_result) => meilisearch_result.hits,
            Err(e) => {
                error!("an error occurred. Err {}", e);
                vec![]
            }
        }
    }
}
