pub mod handler_search_article {
    use std::convert::Infallible;

    use log::{error, info};

    use common::models::article::{ArticleSearchResult, SearchArticleRequest};

    use crate::search_helper::mod_search_helper::{get_authentication_entry, search_index_docs};
    use crate::search_helper_prices::search_helper::{get_movie_customerprice, get_movie_price};

    pub async fn search_article(
        req: SearchArticleRequest,
        engine: String,
    ) -> Result<impl warp::Reply, Infallible> {
        info!(
            "start search_article(). search_text '{}', offset {}, limit {}, engine {}",
            req.q, req.offset, req.limit, &engine
        );

        let customer = get_authentication_entry(&req.customer).await;
        if customer.is_none() {
            let id = &req.customer.customer_id.map_or(-1, |i| i);
            error!("customer {} is not logged in (-1 if no id provided", id);
        }
        let search_result = search_index_docs(&engine, &req.q, req.limit, req.offset).await;

        if search_result.is_none() {
            return Ok(warp::reply::json::<Vec<ArticleSearchResult>>(&vec![]));
        }

        let mut search_result = search_result.unwrap();
        let mut res = vec![];

        for m in search_result.movies.drain(..) {
            let price = get_movie_price(&m.tconst).await;
            if price.is_none() {
                error!("no price found for movie tconst {}", &m.tconst);
                continue;
            }
            let price = price.map(|p| p.amount).unwrap();
            let customer_price = match &customer {
                Some(aa) => {
                    if m.year.is_some() {
                        let a =
                            get_movie_customerprice(m.year.unwrap() as i32, aa.customer_id).await;
                        a.map(|c| (100.0 - c.discount) * price / 100.0)
                    } else {
                        info!("year not available on movie  -> no customer prize");
                        None
                    }
                }
                None => {
                    let id = &req.customer.customer_id.map_or(-1, |i| i);
                    error!("customer customer.id {} not authenticated", id);
                    None
                }
            };
            let a = ArticleSearchResult {
                article: m,
                price,
                customer_price,
            };
            res.push(a);
        }

        Ok(warp::reply::json(&res))
    }
}
