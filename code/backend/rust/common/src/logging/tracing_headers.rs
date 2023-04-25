pub mod tracing_headers_stuff {
    use std::time::Instant;

    use log::error;
    use reqwest::header::{HeaderMap, HeaderValue};
    use reqwest::StatusCode;
    use serde::Serialize;
    use serde_json::json;
    use uuid::Uuid;
    use warp::Reply;
    use warp::reply::Response;

    pub const HEADER_X_INITIATED_BY: &str = "X-initiated-by";
    pub const HEADER_X_PROCESSED_BY: &str = "X-processed-by";
    pub const HEADER_X_UUID: &str = "X-uuid";

    pub fn get_trace_infos(headers: &HeaderMap, service_name: String) -> (String, String, String) {
        let mut initiated_by = None;
        let mut uuid = None;
        let mut processed_by = None;
        if headers.contains_key(HEADER_X_INITIATED_BY) {
            initiated_by = headers.get(HEADER_X_INITIATED_BY);
        }
        if headers.contains_key(HEADER_X_UUID) {
            uuid = headers.get(HEADER_X_UUID);
        };
        if headers.contains_key(HEADER_X_PROCESSED_BY) {
            processed_by = headers.get(HEADER_X_PROCESSED_BY);
        };

        let initiated_by = match initiated_by {
            Some(h) => h.to_str().unwrap(),
            None => service_name.as_str(),
        };

        let uuid = match uuid {
            Some(u) => u.to_str().unwrap().to_string(),
            None => {
                let uuid = Uuid::new_v4();
                uuid.to_string()
            }
        };

        let processed_by = match processed_by {
            Some(h) => match h.to_str() {
                Ok(head) => head,
                Err(e) => {
                    error!(
                        "error unwrapping header value {:?}, e {:?} ",
                        processed_by, e
                    );
                    "error unwrapping header value"
                }
            },
            None => service_name.as_str(),
        };

        (initiated_by.to_string(), uuid, processed_by.to_string())
    }

    pub fn build_tracing_headers(
        start_total: &Instant,
        service_name: &String,
        initiated_by: &String,
        uuid: &String,
        processed_by: &String,
        msg: &String,
    ) -> HeaderMap {
        let duration_total = start_total.elapsed();

        let mut headers = HeaderMap::new();

        headers.insert(
            HEADER_X_INITIATED_BY,
            HeaderValue::from_str(&initiated_by).unwrap(),
        );

        headers.insert(HEADER_X_UUID, HeaderValue::from_str(&uuid).unwrap());

        let new_x_processed_by = format!(
            " {}: dur {:?} micros # {} || {}",
            service_name,
            duration_total.as_micros(),
            &msg,
            processed_by,
        );

        headers.insert(
            HEADER_X_PROCESSED_BY,
            HeaderValue::from_str(&new_x_processed_by).unwrap(),
        );

        headers
    }

    pub fn build_response_from_json<T: Serialize>(json: T, headers: HeaderMap) -> Response {
        build_response_from_json_with_status(json, headers, StatusCode::OK)
    }

    pub fn build_response_from_json_with_status<T: Serialize>(
        json: T,
        headers: HeaderMap,
        status: StatusCode,
    ) -> Response {
        let value = json!(&json);
        let reply = warp::reply::json(&value);
        let reply = warp::reply::with_status(reply, status);

        let mut response = reply.into_response();
        response.headers_mut().extend(headers);
        response
    }
}
