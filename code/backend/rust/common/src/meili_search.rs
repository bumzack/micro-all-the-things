pub mod handlers_search_entity {
    use std::convert::Infallible;

    use reqwest::{Client, Error, Response, StatusCode};
    use serde_json::json;
    use warp::hyper;

    use crate::crew::Crew;
    use crate::episode::Episode;
    use crate::logging_service_client::logging_service;
    use crate::movie::Movie;
    use crate::movieaka::MovieAkas;
    use crate::person::Person;
    use crate::search::{MeiliSearchRequest, MeiliSearchResult, SearchPaginatedRequest};
    use crate::search_doc::SearchIndexDoc;

    async fn meili_search_paginated(
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
            .body(json)
            .header("Authorization", "Bearer 1234567890123456".to_owned())
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        dump_response_status(&response);

        response
    }

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

        let response2 = response.await.unwrap();

        let result = response2.json::<MeiliSearchResult<SearchIndexDoc>>().await;

        let doc = match result {
            Ok(doc) => {
                let msg = format!(
                    "finished meili_search_searchindex(). search_text '{}'. limit {}, offset {}, facets {:?}.  returned {} index_documents ",
                    search_text,
                    limit, offset, facets,
                    doc.hits.len()
                );
                logging_service::log_entry("meili_search".to_string(), "INFO".to_string(), &msg)
                    .await;

                Ok(warp::reply::with_status(
                    json!(&doc).to_string(),
                    StatusCode::OK,
                ))
            }
            Err(e) => {
                let msg = format!(
                    "finished meili_search_searchindex(). search_text '{}'. limit {}, offset {}, facets {:?}.  returned an error {} ",
                    search_text,
                    limit, offset, facets,
                    e
                );
                logging_service::log_entry("meili_search".to_string(), "ERROR".to_string(), &msg)
                    .await;
                Ok(warp::reply::with_status(
                    e.to_string(),
                    StatusCode::NOT_FOUND,
                ))
            }
        };
        doc
    }

    pub async fn meili_search_person(
        entity: String,
        search_text: String,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_search(entity, search_text, client);

        let response2 = response.await.unwrap();
        let result = response2
            .json::<MeiliSearchResult<Person>>()
            .await
            .expect("expected a MeiliSearchResult");

        let persons = result.hits;

        Ok(warp::reply::json(&persons))
    }

    pub async fn meili_search_movie(
        entity: String,
        search_text: String,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_search(entity, search_text.clone(), client);

        let response2 = response.await.unwrap();
        let result = response2
            .json::<MeiliSearchResult<Movie>>()
            .await
            .expect("expected a MeiliSearchResult");

        let movies = result.hits;

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

    pub async fn meili_search_movieaka(
        entity: String,
        search_text: String,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_search(entity, search_text, client);

        let response2 = response.await.unwrap();
        let result = response2
            .json::<MeiliSearchResult<MovieAkas>>()
            .await
            .expect("expected a MeiliSearchResult");

        let persons = result.hits;

        Ok(warp::reply::json(&persons))
    }

    pub async fn meili_search_crew(
        entity: String,
        search_text: String,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_search(entity, search_text, client);

        let response2 = response.await.unwrap();
        let result = response2
            .json::<MeiliSearchResult<Crew>>()
            .await
            .expect("expected a MeiliSearchResult");

        let persons = result.hits;

        Ok(warp::reply::json(&persons))
    }

    pub async fn meili_search_episode(
        entity: String,
        search_text: String,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_search(entity, search_text, client);

        let response2 = response.await.unwrap();
        let result = response2
            .json::<MeiliSearchResult<Episode>>()
            .await
            .expect("expected a MeiliSearchResult");

        let persons = result.hits;

        Ok(warp::reply::json(&persons))
    }

    async fn meili_search(
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

        let response = client
            .post(&index)
            .body(json)
            .header("Authorization", "Bearer 1234567890123456".to_owned())
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        dump_response_status(&response);

        response
    }

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
            .body(json)
            .header("Authorization", "Bearer 1234567890123456".to_owned())
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        dump_response_status(&response);

        let response2 = response.unwrap();
        let result = response2.json::<MeiliSearchResult<Movie>>().await;
        let movies = match result {
            Ok(r) => r.hits,
            Err(e) => {
                error!("an error occurred sending a paginated search request to the meilisearch server {} ",e);
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
            .body(json)
            .header("Authorization", "Bearer 1234567890123456".to_owned())
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        dump_response_status(&response);

        // 🙏 https://github.com/seanmonstar/warp/issues/38
        let stream = response.unwrap().bytes_stream();
        let body = hyper::Body::wrap_stream(stream);
        Ok(warp::reply::Response::new(body))
    }

    pub fn dump_response_status(response: &Result<Response, Error>) {
        match &response {
            Ok(res) => {
                let code = res.status().clone();
                if code == StatusCode::OK
                    || code == StatusCode::ACCEPTED
                    || code == StatusCode::CREATED
                {
                    info!("request success");
                } else {
                    let x = res.headers().clone();
                    // let b = res.text().await.unwrap();
                    error!("request != OK. status {:?}", code);
                    error!("request != OK. headers {:?}", x);
                    // info!("meilisearch search request != OK. response body {:?}", &b);
                }
            }
            Err(e) => error!("error in request to meilisearch {:?}", e),
        };
    }
}
