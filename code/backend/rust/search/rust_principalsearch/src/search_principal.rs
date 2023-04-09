pub mod filters_search_movie {
    use std::convert::Infallible;

    use log::info;
    use reqwest::Client;
    use warp::{Filter, Reply};

    use common::meili::meili_filter::meili_filter_principal::meili_filter_principal_vec;
    use common::solr::solr_filter::solr_filter_principal::solr_filter_principal_vec;

    use crate::CLIENT;

    pub fn filter_principal_route() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
        let server = warp::path!("api" / "meili" / "principal" / "filter" / "name" / String);
        let search_name_meili = server.and(warp::get()).and_then(|nconst: String| {
            info!("/api/meili/principal/filter/name/:nconst     matched");
            filter_principal("nconst".to_string(), nconst, "meili".to_string(), &CLIENT)
        });

        let server2 = warp::path!("api" / "meili" / "principal" / "filter" / "title" / String);
        let filter_title_meili = server2.and(warp::get()).and_then(|tconst: String| {
            info!("/api/meili/principal/filter/title/:tconst     matched");
            filter_principal("tconst".to_string(), tconst, "meili".to_string(), &CLIENT)
        });

        let server = warp::path!("api" / "solr" / "principal" / "filter" / "name" / String);
        let search_name_solr = server.and(warp::get()).and_then(|nconst: String| {
            info!("/api/solr/principal/filter/name/:nconst     matched");
            filter_principal("nconst".to_string(), nconst, "solr".to_string(), &CLIENT)
        });

        let server2 = warp::path!("api" / "solr" / "principal" / "filter" / "title" / String);
        let filter_title_solr = server2.and(warp::get()).and_then(|tconst: String| {
            info!("/api/solr/principal/filter/title/:tconst     matched");
            filter_principal("tconst".to_string(), tconst, "solr".to_string(), &CLIENT)
        });

        filter_title_meili
            .or(filter_title_solr)
            .or(search_name_meili)
            .or(search_name_solr)
    }

    pub async fn filter_principal(
        filter_field: String,
        filter_value: String,
        engine: String,
        client: &Client,
    ) -> Result<impl Reply, Infallible> {
        let principals = match engine.as_str() {
            "solr" => solr_filter_principal_vec(filter_field, vec![filter_value], client).await,
            "meili" => meili_filter_principal_vec(filter_field, vec![filter_value], client).await,
            _ => vec![],
        };

        Ok(warp::reply::json(&principals))
    }
}
