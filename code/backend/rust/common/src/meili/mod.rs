use log::{error, info};
use reqwest::{Error, Response, StatusCode};

pub mod meili;
pub mod meili_filter;
pub mod meili_models;
pub mod meili_read_docs;
pub mod meili_search;

pub fn dump_response_status(
    response: &Result<Response, Error>,
    url: &String,
    json: &String,
    engine: String,
) {
    match &response {
        Ok(res) => {
            let code = res.status();
            if code == StatusCode::OK || code == StatusCode::ACCEPTED || code == StatusCode::CREATED
            {
                info!("request success");
            } else {
                let x = res.headers().clone();
                error!("request != OK. status {:?},    url {}", code, url);
                error!("request != OK. headers {:?},    url {}", x, url);
                error!("remote address {:?}", res.remote_addr());
            }
        }
        Err(e) => error!(
            "request to {} resulted in an error. request URL '{}', json '{}' error '{:?}'",
            engine, url, json, e
        ),
    };
}
