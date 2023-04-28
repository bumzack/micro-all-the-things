pub mod filters_tsv {
    use warp::Filter;

    use common::tsv::tsv::TsvFileImportRequest;

    use super::handlers_tsv;

    pub fn tsv_request_route(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path("tsv").and(tsv_post())
    }

    pub fn tsv_post() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
    {
        warp::path!("read")
            .and(warp::post())
            .and(json_body_tsv_request())
            .and_then(handlers_tsv::post_tsv_request)
    }

    fn json_body_tsv_request(
    ) -> impl Filter<Extract = (TsvFileImportRequest,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers_tsv {
    use std::convert::Infallible;

    use log::info;
    use serde_json::json;
    use tokio::fs::File;
    use tokio::io::{AsyncBufReadExt, BufReader};

    use common::tsv::tsv::{TsvFileImportRequest, TsvLine, TsvLines};

    use crate::{CLIENT, CONFIG};

    pub async fn post_tsv_request(
        tsv_request: TsvFileImportRequest,
    ) -> Result<impl warp::Reply, Infallible> {
        let msg = format!(
            "start post_tsv_request. tsvFileImportRequest. entity {:?}, start {}, end {}, page_size {}",
            &tsv_request.tsv_type.clone(), tsv_request.start, tsv_request.end, tsv_request.page_size
        );

        info!("tsv_file_import_request {:?}", &tsv_request);

        let start = tsv_request.start as usize;
        let page_size = tsv_request.page_size as u64;
        let end = tsv_request.end as u64;
        let t = tsv_request.tsv_type;

        let filename_property = format!("datasource_{:?}_filename", t);
        // info!("filename_property {}", &filename_property);
        let filename: String = CONFIG
            .get(&filename_property)
            .expect("filename_property must exist");
        let datasource_folder: String = CONFIG
            .get("datasource_folder")
            .expect("datasource_folder must exist");

        let filename = format!("{}/{}", datasource_folder, filename);
        // info!("filename {}", &filename);
        let file = File::open(&filename).await;
        let file = match file {
            Ok(f) => f,
            Err(err) => {
                let msg = format!(
                    "error in post_tsv_request. can't open file  {:?}. error  {:?}",
                    &filename, err
                );

                info!("error opening file {}. err {}", &filename, err);
                let ret = format!("error opening file {}", &filename);
                return Ok(warp::reply::json(&ret));
            }
        };
        let reader = BufReader::new(file);

        let target_host_property = format!("microservice_{:?}_host", t);
        let host: String = CONFIG
            .get(&target_host_property)
            .expect("target_host_property must exist");
        let target_port_property = format!("microservice_{:?}_port", t);
        let port: String = CONFIG
            .get(&target_port_property)
            .expect("target_port_property must exist");
        let target_url_property = format!("microservice_{:?}_url", t);
        let url: String = CONFIG
            .get(&target_url_property)
            .expect("target_host_property must exist");

        let request_url = format!("{}:{}{}", host, port, url);
        // info!("request_url {}", &request_url);

        // FUNNY NICE DEMO BUG
        //  while let Ok(l) = reader.lines().next_line().await {

        let mut lines = reader.lines();
        let mut batches = 0;
        // skip to start
        let mut current_line = 0;
        if start > 0 {
            while let Ok(_l) = lines.next_line().await {
                current_line += 1;
                if current_line == start {
                    break;
                }
            }
        }

        current_line = start;

        let mut stuff_available = true;
        while current_line <= end as usize && stuff_available {
            let mut tsv_lines = vec![];
            let mut idx = 0;
            while let Ok(l) = lines.next_line().await {
                idx += 1;
                current_line += 1;

                if l.is_none() {
                    info!("we are done here ");
                    break;
                }

                let line = l.unwrap();
                if line.is_empty() {
                    info!("line is empty -> skipping");
                    break;
                }

                let entries = line.clone().split('\t').map(|s| s.to_string()).collect();
                let tsv = TsvLine {
                    entries,
                    original: line,
                };
                tsv_lines.push(tsv);

                if idx > page_size - 1 || (current_line > end as usize) {
                    break;
                }
            }
            batches += 1;

            let request_url = request_url.clone();

            let tsv_lines = TsvLines { lines: tsv_lines };

            if tsv_lines.lines.is_empty() {
                info!("no tsv_lines available -> breaking in while");
                stuff_available = false;
            }

            let json = json!(&tsv_lines).to_string();

            info!("request url  '{}'", &request_url);
            let res = CLIENT
                .post(&request_url)
                .header("Content-Type", "application/json".to_owned())
                .body(json)
                .send()
                .await;

            match res {
                Ok(res) => {
                    let code = res.status();
                    let header = res.headers().clone();
                    // let b = res.text().await.unwrap();
                    info!("request ok. status {:?}", code);
                    info!("request ok. headers {:?}", header);
                    // info!("request ok. response body {:?}", &b);
                }
                Err(e) => {
                    info!("error in request {:?}", e);
                    let msg = format!(
                        "error in post_tsv_request. calling URL {:?} resulted in error  {:?}",
                        &request_url, e
                    );
                }
            }

            info!("processed batch {} for type {:?}", batches, &t);

            let msg = format!(
                "start post_tsv_request. processed {} batches for entity {:?}",
                batches, &t
            );

            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        }
        info!("processed lines {}", current_line);

        let msg = format!(
            "end post_tsv_request. processed {:?} batches of entity {:?}",
            batches, &t
        );

        let res = format!("all good. processed {} batches", batches);
        Ok(warp::reply::json(&res))
    }
}
