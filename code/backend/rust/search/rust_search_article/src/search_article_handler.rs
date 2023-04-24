pub mod handler_search_article {
    use log::{error, info};

    use common::entity::entity::Engine;
    use common::models::article::{
        ArticleSearchResult, SearchArticleRequest, SearchArticleResponse,
    };

    use crate::search_helper::mod_search_helper::{get_authentication_entry, search_index_docs};
    use crate::search_helper_prices::search_helper::{get_movie_customerprice, get_movie_price};

    pub async fn search_article(
        req: SearchArticleRequest,
        engine: Engine,
        initiated_by: &String,
        uuid: &String,
        processed_by: &String,
    ) -> (SearchArticleResponse, String) {
        info!(
            "start search_article(). search_text '{}', offset {}, limit {}, engine {:?}",
            req.q, req.offset, req.limit, engine
        );

        let (authentication_entry, n) =
            get_authentication_entry(&req.customer, initiated_by, uuid, processed_by).await;
        let mut new_processed_by = n;

        if authentication_entry.is_none() {
            let id = &req.customer.customer_id.map_or(-1, |i| i);
            error!("customer {} is not logged in (-1 if no id provided", id);
        }

        info!("search_auth   calling 'search_index_docs'");
        info!(
            "new_processed_by before   search_index_docs  {}",
            &new_processed_by
        );

        let (search_result, n) = search_index_docs(
            engine,
            &req.q,
            req.limit,
            req.offset,
            initiated_by,
            uuid,
            &new_processed_by,
        )
            .await;
        new_processed_by = n;

        info!(
            "new_processed_by after   search_index_docs  {}",
            &new_processed_by
        );

        if search_result.is_none() {
            info!("search_index_docs   no search result found -> returning empty array");
            let r = SearchArticleResponse {
                articles: None,
                facets: None,
            };
            return (r, new_processed_by.clone());
        }

        let mut search_result = search_result.unwrap();
        let mut res = vec![];

        for m in search_result.movies.drain(..) {
            info!(
                "search_article  before  get_movie_price. tconst  {}",
                &m.tconst
            );
            let (price, n) =
                get_movie_price(&m.tconst, initiated_by, uuid, &new_processed_by).await;
            new_processed_by = n;

            info!(
                "search_article  after  get_movie_price. tconst  {},  new_processed_by  {}",
                &m.tconst, new_processed_by
            );
            if price.is_none() {
                error!("no price found for movie tconst {}", &m.tconst);
                continue;
            } else {
                info!("search_article  found price for movie  {}", &m.tconst);
            }
            let price = price.map(|p| p.amount).unwrap();
            let (customer_price, n) = match &authentication_entry {
                Some(auth_entry) => {
                    if m.year.is_some() {
                        let (customer_price, n) = get_movie_customerprice(
                            m.year.unwrap() as i32,
                            auth_entry.customer_id,
                            initiated_by,
                            uuid,
                            &new_processed_by,
                        )
                            .await;

                        info!(
                            "new_processed_by  after  get_movie_customerprice. new_processed_by  {}",
                            new_processed_by
                        );

                        new_processed_by = n;

                        info!(
                            "search_article  found a customer price for movie  {}, customer {}",
                            &m.tconst, &auth_entry.customer_id
                        );

                        (
                            customer_price.map(|c| (100.0 - c.discount) * price / 100.0),
                            new_processed_by.to_string(),
                        )
                    } else {
                        info!("year not available on movie  -> no customer prize");
                        (None, new_processed_by.to_string())
                    }
                }
                None => {
                    let id = &req.customer.customer_id.map_or(-1, |i| i);
                    error!("customer customer.id {} not authenticated", id);
                    (None, new_processed_by.to_string())
                }
            };
            new_processed_by = n;
            let a = ArticleSearchResult {
                article: m,
                price,
                customer_price,
            };
            info!("search_article  added final ArticleSearchResult to vec");

            res.push(a);
        }

        let res = SearchArticleResponse {
            articles: Some(res),
            facets: search_result.facets,
        };

        info!(
            "search_article  final  new_processed_by {}",
            new_processed_by
        );
        (res, new_processed_by)
    }
}
