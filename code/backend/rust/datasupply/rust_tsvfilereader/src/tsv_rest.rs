pub mod filters_tsv {
    use warp::Filter;

    use common::TsvFileImportRequest;

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

    use serde_json::json;
    use tokio::fs::File;
    use tokio::io::{AsyncBufReadExt, BufReader};

    use common::{TsvFileImportRequest, TsvLine, TsvLines};

    use crate::{CLIENT, CONFIG};

    pub async fn post_tsv_request(
        tsv_request: TsvFileImportRequest,
    ) -> Result<impl warp::Reply, Infallible> {
        println!("tsv_file_import_request {:?}", &tsv_request);

        // println!(
        //     "tsv_file_import_request.tsv_type          {:?}",
        //     &tsv_request.tsv_type
        // );
        // println!(
        //     "tsv_file_import_request.start             {:?}",
        //     &tsv_request.start
        // );
        // println!(
        //     "tsv_file_import_request.page_size         {:?}",
        //     &tsv_request.page_size
        // );
        // println!(
        //     "tsv_file_import_request.end               {:?}",
        //     &tsv_request.end
        // );

        let start = tsv_request.start as usize;
        let page_size = tsv_request.page_size as u64;
        let end = tsv_request.end as u64;
        let t = tsv_request.tsv_type;

        let filename_property = format!("datasource_{:?}_filename", t);
        // println!("filename_property {}", &filename_property);
        let filename: String = CONFIG
            .get(&filename_property)
            .expect("filename_property must exist");
        let datasource_folder: String = CONFIG
            .get("datasource_folder")
            .expect("datasource_folder must exist");

        let filename = format!("{}/{}", datasource_folder, filename);
        // println!("filename {}", &filename);
        let file = File::open(&filename).await;
        let file = match file {
            Ok(f) => f,
            Err(err) => {
                println!("error opening file {}. err {}", &filename, err);
                let ret = format!("error opening file {}", &filename);
                return Ok(warp::reply::json(&ret));
            }
        };
        let reader = BufReader::new(file);

        let target_host_property = format!("microservice_{:?}_host", t);
        // println!("target_host_property {}", &target_host_property);
        let host: String = CONFIG
            .get(&target_host_property)
            .expect("target_host_property must exist");
        let target_port_property = format!("microservice_{:?}_port", t);
        //  println!("target_port_property {}", &target_port_property);
        let port: String = CONFIG
            .get(&target_port_property)
            .expect("target_port_property must exist");
        let target_url_property = format!("microservice_{:?}_url", t);
        //  println!("target_url_property {}", &target_url_property);
        let url: String = CONFIG
            .get(&target_url_property)
            .expect("target_host_property must exist");

        // println!("host {}", host);
        // println!("port {}", port);
        // println!("url  {}", url);

        let request_url = format!("{}:{}{}", host, port, url);
        // println!("request_url {}", &request_url);

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
                    println!("we are done here ");
                    break;
                }

                let line = l.unwrap();
                if line.is_empty() {
                    println!("line is empty -> skipping");
                    break;
                }

                let entries = line.clone().split("\t").map(|s| s.to_string()).collect();
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
                println!("no tsv_lines available -> breaking in while");
                stuff_available = false;
            }

            // println!("tsv {:?}", &tsv);

            let json = json!(&tsv_lines).to_string();

            println!("request url  '{}'", &request_url);
            //  let client = reqwest::Client::new();
            let res = CLIENT
                .post(&request_url)
                .header("Content-Type", "application/json".to_owned())
                .body(json)
                .send()
                .await;

            match res {
                Ok(res) => {
                    let code = res.status().clone();
                    //let x = res.headers().clone();
                    // let b = res.text().await.unwrap();
                    println!("request ok. status {:?}", code);
                    //  println!("request ok. headers {:?}", x);
                    // println!("request ok. response body {:?}", &b);
                }
                Err(e) => println!("error in request {:?}", e),
            }

            println!("processed batch {} for type {:?}", batches, &t);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        println!("processed lines {}", current_line);

        let res = format!("all good. processed {} batches", batches);
        Ok(warp::reply::json(&res))
    }
}
