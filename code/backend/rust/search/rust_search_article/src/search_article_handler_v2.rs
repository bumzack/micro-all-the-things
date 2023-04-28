pub mod handler_search_article_v2 {
    use std::collections::HashMap;

    use log::{error, info};

    use common::entity::entity::Engine;
    use common::models::article::{
        ArticleSearchResult, SearchArticleRequest, SearchArticleResponse,
    };
    use common::models::prices::PriceEntry;
    use common::models::search_doc::MovieSearchResult;

    use crate::search_helper::mod_search_helper::{get_authentication_entry, search_index_docs};
    use crate::search_helper_prices::search_helper::{get_movie_customerprices, get_movie_prices};

    pub async fn search_article_v2(
        req: SearchArticleRequest,
        engine: Engine,
        initiated_by: &String,
        uuid: &String,
        processed_by: &String,
    ) -> (SearchArticleResponse, String) {
        info!(
            "start search_article_v2(). search_text '{}', offset {}, limit {}, engine {:?}",
            req.q, req.offset, req.limit, engine
        );

        let (authentication_entry, n) =
            get_authentication_entry(&req.customer, initiated_by, uuid, processed_by).await;
        let mut new_processed_by = n;

        if authentication_entry.is_none() {
            let id = &req.customer.customer_id.map_or(-1, |i| i);
            error!("customer {} is not logged in (-1 if no id provided", id);
        }

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

        let (prices, n) =
            find_movie_prices(initiated_by, uuid, &new_processed_by, &search_result).await;
        new_processed_by = n;

        let customer_prices = match &authentication_entry {
            // why, what's happening ...
            Some(auth_entry) => {
                let (customer_prices, n) = get_movie_customerprices(
                    auth_entry.customer_id,
                    initiated_by,
                    uuid,
                    &new_processed_by,
                )
                .await;
                new_processed_by = n;

                info!(
                    "new_processed_by  after  get_movie_customerprice. new_processed_by  {}",
                    new_processed_by
                );
                customer_prices
            }
            None => {
                let id = &req.customer.customer_id.map_or(-1, |i| i);
                error!("customer customer.id {} not authenticated", id);
                None
            }
        };

        for m in search_result.movies.drain(..) {
            let movie_price = prices.get(&m.tconst);
            let mut customer_price = None;

            // oh boy o boy oh body - what a mess
            if movie_price.is_some() && customer_prices.is_some() {
                let customer_prices = customer_prices.as_ref().unwrap();
                if m.year.is_some() {
                    let year = m.year.unwrap();
                    customer_price = customer_prices
                        .iter()
                        .find(|pr| pr.start_year <= year as i32 && year as i32 <= pr.end_year);
                }
            }

            let price = movie_price.map_or(0.0, |p| p.amount);

            // oh boy
            let customer_price = customer_price
                .map(|cp| (100.0 - cp.discount) * price)
                .or(None);

            let a = ArticleSearchResult {
                article: m,
                price,
                customer_price,
            };
            info!("search_article_v2  added final ArticleSearchResult to vec");

            res.push(a);
        }

        let res = SearchArticleResponse {
            articles: Some(res),
            facets: search_result.facets,
        };

        info!(
            "search_article_v2  final  new_processed_by {}",
            new_processed_by
        );
        (res, new_processed_by)
    }

    async fn find_movie_prices(
        initiated_by: &String,
        uuid: &String,
        processed_by: &String,
        search_result: &MovieSearchResult,
    ) -> (HashMap<String, PriceEntry>, String) {
        let mut new_processed_by = processed_by.clone();

        let mut movies_processed = 0;
        let page_size = 50;
        let mut idx = 0;

        let mut movie_prices: HashMap<String, PriceEntry> = HashMap::new();
        while movies_processed < search_result.movies.len() {
            let mut tconsts = vec![];
            search_result
                .movies
                .iter()
                .skip(idx)
                .take(page_size)
                .for_each(|m| tconsts.push(m.tconst.clone()));
            let (prices, n) =
                get_movie_prices(tconsts, initiated_by, uuid, &new_processed_by).await;
            new_processed_by = n;
            if prices.is_none() {
                info!("search_article_v2   no prices found for movies -> returning empty HashMap");
                return (HashMap::new(), new_processed_by.clone());
            }

            let prices = prices.unwrap();

            prices.iter().for_each(|p| {
                let _ = movie_prices.insert(p.movie_tconst.clone(), (*p).clone());
            });

            movies_processed += page_size;
            idx += page_size;
        }

        info!(
            "search_article_v2  got  {} movie_prices for {} movies",
            &movie_prices.len(),
            search_result.movies.len()
        );

        if movie_prices.len() != search_result.movies.len() {
            error!("missing some prices for movies");
        }
        (movie_prices, new_processed_by)
    }
}
