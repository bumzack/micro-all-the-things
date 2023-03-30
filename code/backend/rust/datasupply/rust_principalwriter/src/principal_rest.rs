pub mod filters_principal {
    use super::handlers_principal;
    use common::TsvLine;
    use warp::Filter;

    pub fn principal_route(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("api").and(principal_post())
    }

    pub fn principal_post(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("principal")
            .and(warp::post())
            .and(json_body_tsv_line())
            .and_then(handlers_principal::post_principal)
    }

    fn json_body_tsv_line() -> impl Filter<Extract = (TsvLine,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers_principal {
    use std::convert::Infallible;

    use serde_json::json;

    use crate::CLIENT;
    use common::{Principal, TsvLine};

    pub async fn post_principal(tsv_line: TsvLine) -> Result<impl warp::Reply, Infallible> {
        let ordering = match tsv_line.entries.get(1) {
            Some(ordering) => ordering.parse::<u32>().unwrap(),
            None => panic!("should not happen"),
        };

        let characters = match tsv_line.entries.get(4) {
            Some(characters) => characters.split(",").map(|s| s.to_string()).collect(),
            None => panic!("should not happen"),
        };
        let tconst = tsv_line.entries.get(0).unwrap().to_string();
        let nconst = tsv_line.entries.get(2).unwrap().to_string();
        let id = format!("{}_{}_{}", tconst, ordering, nconst);
        let category = tsv_line.entries.get(3).unwrap().to_string();

        let principal = Principal {
            id,
            tconst,
            ordering,
            nconst,
            category,
            characters,
        };

        let json = json!(&principal).to_string();

        // println!("json \n {}\n", &json);

        // let client = reqwest::Client::new();
        let response = CLIENT
            .post("http://meilisearch01.bumzack.at/indexes/principal/documents?primaryKey=id")
            .body(json)
            .header("Authorization", "Bearer 1234567890123456".to_owned())
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        match response {
            Ok(_res) => {
                // let code = res.status().clone();
                // let x = res.headers().clone();
                // let b = res.text().await.unwrap();
                // println!("request ok. status {:?}", code);
                // println!("request ok. headers {:?}", x);
                // println!("request ok. response body {:?}", &b);
            }
            Err(e) => println!("error in request {:?}", e),
        }
        let res = "dont know".to_string();
        Ok(warp::reply::json(&res))
    }
}
