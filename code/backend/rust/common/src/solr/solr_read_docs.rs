pub mod solr_read_doc {
    use log::error;
    use reqwest::Client;

    use crate::entity::entity::Entity;
    use crate::models::movie::Movie;
    use crate::solr::models::SolrResponse;
    use crate::solr::solr::mod_solr::solr_search;

    pub async fn solr_read_doc_movie_vec(offset: u32, limit: u32, client: &Client) -> Vec<Movie> {
        let sort = vec![("id".to_string(), true)];
        let response = solr_search(
            Entity::MOVIE,
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
        let result = response2.unwrap().json::<SolrResponse<Movie>>().await;
        if result.is_err() {
            error!(
                "cant unwrap response to SolrResponse<Movie> type. error {}",
                result.err().unwrap()
            );
            return vec![];
        }
        let result = result.unwrap();

        let movies = result.response;
        let movies = match movies {
            Some(m) => m.docs.unwrap(),
            None => vec![],
        };
        movies
    }
}
