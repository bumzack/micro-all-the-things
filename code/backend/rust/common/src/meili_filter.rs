pub mod handlers_search_entity {
    use std::convert::Infallible;

    use reqwest::{Client, Error, Response};
    use serde_json::json;

    use crate::crew::Crew;
    use crate::meili_search::handlers_search_entity::dump_response_status;
    use crate::person::Person;
    use crate::principal::Principal;
    use crate::rating::Rating;
    use crate::search::{MeiliSearchRequest, MeiliSearchResult};

    pub async fn meili_filter_person(
        entity: String,
        filter: Vec<String>,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_filter(entity, filter, client);

        let response2 = response.await.unwrap();
        let res = response2.json::<MeiliSearchResult<Person>>().await;

        let persons = match res {
            Ok(res) => res.hits,
            Err(e) => {
                error!("error in request for persons {}", e);
                vec![]
            }
        };

        Ok(warp::reply::json(&persons))
    }

    pub async fn meili_filter_principal(
        entity: String,
        filter: Vec<String>,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_filter(entity, filter, client);

        let response2 = response.await.unwrap();
        let result = response2.json::<MeiliSearchResult<Principal>>().await;

        let principals = match result {
            Ok(res) => res.hits,
            Err(e) => {
                error!("error in request for principals {}", e);
                vec![]
            }
        };

        Ok(warp::reply::json(&principals))
    }

    pub async fn meili_filter_rating(
        entity: String,
        filter: Vec<String>,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_filter(entity, filter, client);

        let response2 = response.await.unwrap();
        let result = response2.json::<MeiliSearchResult<Rating>>().await;

        let ratings = match result {
            Ok(res) => res.hits,
            Err(e) => {
                error!("error in request for ratings {}", e);
                vec![]
            }
        };

        Ok(warp::reply::json(&ratings))
    }

    pub async fn meili_filter_crew(
        entity: String,
        filter: Vec<String>,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_filter(entity, filter, client);

        let response2 = response.await.unwrap();
        let result = response2.json::<MeiliSearchResult<Crew>>().await;

        let crew = match result {
            Ok(res) => res.hits,
            Err(e) => {
                error!("error in request for crew {}", e);
                vec![]
            }
        };

        Ok(warp::reply::json(&crew))
    }

    async fn meili_filter(
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
            filter: Some(filter),
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
            .body(json.clone())
            .header("Authorization", "Bearer 1234567890123456".to_owned())
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        dump_response_status(&response, &index, &json);

        response
    }
}
