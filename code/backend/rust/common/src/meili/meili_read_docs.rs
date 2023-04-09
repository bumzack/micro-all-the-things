pub mod meilisearch_read_doc {
    use std::convert::Infallible;

    use reqwest::Client;

    use crate::entity::entity::Entity;
    use crate::meili::meili::mod_meili::meili_read_doc;
    use crate::meili::meili_models::MeiliDocReadResult;
    use crate::models::movie::Movie;

    pub async fn meili_search_read_doc_movie(
        offset: u32,
        limit: u32,
        client: &Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let movies = meili_search_read_doc_movie_vec(offset, limit, client).await;

        Ok(warp::reply::json(&movies))
    }

    pub async fn meili_search_read_doc_movie_vec(
        offset: u32,
        limit: u32,
        client: &Client,
    ) -> Vec<Movie> {
        let response = meili_read_doc(Entity::MOVIE, offset, limit, client);

        let response2 = response.await.unwrap();
        let result = response2
            .json::<MeiliDocReadResult<Movie>>()
            .await
            .expect("expected a MeiliSearchResult");

        let movies = result.results;
        movies
    }
}
