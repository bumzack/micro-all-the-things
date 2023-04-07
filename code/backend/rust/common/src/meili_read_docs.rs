pub mod meilisearch_read_doc {
    use std::convert::Infallible;

    use reqwest::{Client, Error, Response};

    use crate::meili_search::handlers_search_entity::dump_response_status;
    use crate::movie::Movie;
    use crate::search::MeiliDocReadResult;

    pub async fn meili_search_read_doc_movie(
        offset: u32,
        limit: u32,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let response = meili_read_doc("movie".to_string(), offset, limit, client);

        let response2 = response.await.unwrap();
        let result = response2
            .json::<MeiliDocReadResult<Movie>>()
            .await
            .expect("expected a MeiliSearchResult");

        let movies = result.results;

        Ok(warp::reply::json(&movies))
    }

    async fn meili_read_doc(
        entity: String,
        offset: u32,
        limit: u32,
        client: &Client,
    ) -> Result<Response, Error> {
        info!(
            "reading documents for entity {}.  limit {}, offset {}",
            &entity, limit, offset,
        );

        let index = format!(
            "http://meilisearch01.bumzack.at/indexes/{}/documents?limit={}&offset={}",
            &entity, limit, offset
        );

        info!("reading documents for entity {}. url: {}", &entity, &index);
        let response = client
            .get(&index)
            .header("Authorization", "Bearer 1234567890123456".to_owned())
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        dump_response_status(&response, &index, &"none available".to_string());

        response
    }
}
