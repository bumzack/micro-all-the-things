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
        let result = response2
            .json::<MeiliSearchResult<Person>>()
            .await
            .expect("expected a MeiliSearchResult<Person>");

        let persons = result.hits;

        // let p = serde_json::to_string_pretty(&persons).expect("expected a list of persons");
        // println!("filter_personse returned {}", p);

        Ok(warp::reply::json(&persons))
    }

    pub async fn meili_filter_principal(
        entity: String,
        filter: Vec<String>,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_filter(entity, filter, client);

        let response2 = response.await.unwrap();
        let result = response2
            .json::<MeiliSearchResult<Principal>>()
            .await
            .expect("expected a MeiliSearchResult<Principal>");

        let persons = result.hits;

        Ok(warp::reply::json(&persons))
    }

    pub async fn meili_filter_rating(
        entity: String,
        filter: Vec<String>,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_filter(entity, filter, client);

        let response2 = response.await.unwrap();
        let result = response2
            .json::<MeiliSearchResult<Rating>>()
            .await
            .expect("expected a MeiliSearchResult<Principal>");

        let ratings = result.hits;

        Ok(warp::reply::json(&ratings))
    }

    pub async fn meili_filter_crew(
        entity: String,
        filter: Vec<String>,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_filter(entity, filter, client);

        let response2 = response.await.unwrap();
        let result = response2
            .json::<MeiliSearchResult<Crew>>()
            .await
            .expect("expected a MeiliSearchResult<Crew>");

        let persons = result.hits;

        Ok(warp::reply::json(&persons))
    }

    async fn meili_filter(
        entity: String,
        filter: Vec<String>,
        client: &Client,
    ) -> Result<Response, Error> {
        // println!(
        //     "filter entity {}.  filters  '{:?}'", &entity, &filter
        // );

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
            .body(json)
            .header("Authorization", "Bearer 1234567890123456".to_owned())
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        dump_response_status(&response);

        response
    }
}
