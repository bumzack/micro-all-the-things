pub mod filters_principal {
    use warp::Filter;

    use common::TsvLines;

    use super::handlers_principal;

    pub fn principal_route(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("api").and(principal_post())
    }

    pub fn principal_post(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("principal")
            .and(warp::post())
            .and(json_body_tsv_line())
            .and_then(handlers_principal::post_principals)
    }

    fn json_body_tsv_line() -> impl Filter<Extract = (TsvLines,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 1000 * 1000).and(warp::body::json())
    }
}

mod handlers_principal {
    use std::convert::Infallible;

    use serde_json::json;

    use common::{Principal, TsvLine, TsvLines};

    use crate::CLIENT;

    pub async fn post_principals(tsv_lines: TsvLines) -> Result<impl warp::Reply, Infallible> {
        println!("processing request with {} lines", tsv_lines.lines.len());
        let principals: Vec<Principal> = tsv_lines
            .lines
            .into_iter()
            .map(|t| map_to_principal(&t))
            .collect();

        let json = json!(&principals).to_string();

        // let client = reqwest::Client::new();
        let response = CLIENT
            .post("http://meilisearch01.bumzack.at/indexes/principal/documents?primaryKey=id")
            .body(json)
            .header("Authorization", "Bearer 1234567890123456".to_owned())
            .header("Content-Type", "application/json".to_owned())
            .send()
            .await;

        match response {
            Ok(res) => {
                let code = res.status().clone();
                // let x = res.headers().clone();
                // let b = res.text().await.unwrap();
                println!("request ok. status {:?}", code);
                // println!("request ok. headers {:?}", x);
                // println!("request ok. response body {:?}", &b);
            }
            Err(e) => println!("error in request {:?}", e),
        }
        let res = "all good".to_string();
        Ok(warp::reply::json(&res))
    }

    fn map_to_principal(tsv_line: &TsvLine) -> Principal {
        println!("mapping tsv_line {:?}", &tsv_line);

        let ordering = get_nullable_u32(&tsv_line.entries, 1).expect("ordering should be there");

        let characters = get_nullable_string_list(&tsv_line.entries, 4);
        let tconst = get_nullable_string(&tsv_line.entries, 0).unwrap();
        let nconst = get_nullable_string(&tsv_line.entries, 2).unwrap();
        let id = format!("{}_{}_{}", tconst, ordering, nconst);
        let category = get_nullable_string(&tsv_line.entries, 3).unwrap();

        Principal {
            id,
            tconst,
            ordering,
            nconst,
            category,
            characters,
        }
    }

    fn get_nullable_string(input: &Vec<String>, idx: usize) -> Option<String> {
        match input.get(idx) {
            Some(s) => {
                if s.eq("\\N") {
                    return None;
                }
                Some(s.to_string())
            }
            None => {
                panic!("should not happen, that a field is empty")
            }
        }
    }

    fn get_nullable_u32(input: &Vec<String>, idx: usize) -> Option<u32> {
        match input.get(idx) {
            Some(s) => {
                if s.eq("\\N") {
                    return None;
                }
                Some(s.parse::<u32>().unwrap())
            }
            None => {
                panic!("should not happen, that a field is empty")
            }
        }
    }

    fn get_nullable_string_list(input: &Vec<String>, idx: usize) -> Option<Vec<String>> {
        match input.get(idx) {
            Some(s) => {
                if s.eq("\\N") {
                    return None;
                }
                let characters = s.split(",").map(|s| s.to_string()).collect();
                Some(characters)
            }
            None => {
                panic!("should not happen, that a field is empty")
            }
        }
    }
}
