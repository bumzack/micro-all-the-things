use crate::entity::entity::Engine;
use log::{error, info};
use reqwest::{Error, Response, StatusCode};

pub fn dump_response_status(
    response: &Result<Response, Error>,
    url: &String,
    json: &String,
    engine: Engine,
) {
    match &response {
        Ok(res) => {
            let code = res.status();
            if code != StatusCode::OK && code != StatusCode::ACCEPTED && code != StatusCode::CREATED
            {
                let x = res.headers().clone();
                info!("request != OK. status {:?},    url {}", code, url);
                info!("request != OK. headers {:?},    url {}", x, url);
                info!("remote address {:?}", res.remote_addr());
            }
        }
        Err(e) => error!(
            "request to {:?} resulted in an error. request URL '{}', json '{}' error '{:?}'",
            &engine.to_string(),
            url,
            json,
            e
        ),
    };
}
