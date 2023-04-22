pub mod mod_search_helper {
    use log::{error, info};
    use reqwest::StatusCode;

    use common::entity::entity::Engine;
    use common::logging::tracing_headers::tracing_headers_stuff::{
        get_trace_infos, HEADER_X_INITIATED_BY, HEADER_X_PROCESSED_BY, HEADER_X_UUID,
    };
    use common::models::article::SearchCustomer;
    use common::models::authentication::AuthenticationEntry;
    use common::models::search_doc::{MovieSearchResult, SearchMovieIndexRequest};

    use crate::{CLIENT, CONFIG};

    pub async fn get_authentication_entry(
        search_customer: &SearchCustomer,
        initiated_by: &String,
        uuid: &String,
        processed_by: &String,
    ) -> (Option<AuthenticationEntry>, String) {
        let search_auth: String = CONFIG
            .get("customer_authenticated")
            .expect("expected customer_authenticated GET request URL");

        info!("search for customer {:?}", &search_customer);

        match search_customer.customer_id {
            Some(id) => {
                let search_auth = search_auth.replace(":customer_id", &id.to_string());
                info!("search_auth request  {:?}", &search_auth);
                let response = CLIENT
                    .get(search_auth)
                    .header(HEADER_X_PROCESSED_BY, processed_by)
                    .header(HEADER_X_UUID, uuid)
                    .header(HEADER_X_INITIATED_BY, initiated_by)
                    .send()
                    .await;
                if response.is_err() {
                    error!(
                        "error from AuthenticationService {:?}",
                        response.err().unwrap()
                    );
                    return (None, processed_by.to_string());
                }

                info!("search_auth request  got a useful response");

                match response {
                    Ok(res) => {
                        let (processed_by_new, _, _) =
                            get_trace_infos(res.headers(), "get_authentication".to_string());
                        if res.status() == StatusCode::OK {
                            info!("search_auth   response is 200");

                            let auth = res.json::<AuthenticationEntry>().await;

                            match auth {
                                Ok(auth) => {
                                    info!("search_auth   authentication Entry present");
                                    if auth.jwt.is_some()
                                        && auth.logged_in.is_some()
                                        && auth.logged_out.is_none()
                                    {
                                        (Some(auth), processed_by_new)
                                    } else {
                                        (None, processed_by_new)
                                    }
                                }
                                Err(e) => {
                                    error!("authentication service returned an error {:?}", e);
                                    (None, processed_by_new)
                                }
                            }
                        } else {
                            error!("authentication Service returned status {} for customer.id {}. assuming not logged in. ", res.status(), id);
                            (None, processed_by_new)
                        }
                    }
                    Err(e) => {
                        error!("authentication service returned an error {:?}", e);
                        (None, processed_by.to_string())
                    }
                }
            }

            None => {
                error!("no customer id provided ");
                (None, processed_by.to_string())
            }
        }
    }

    pub async fn search_index_docs(
        engine: Engine,
        q: &String,
        limit: u32,
        offset: u32,
        initiated_by: &String,
        uuid: &String,
        processed_by: &String,
    ) -> (Option<MovieSearchResult>, String) {
        let search_index_docs: String = CONFIG
            .get("search_index_doc")
            .expect("expected search_index_doc GET request URL");

        let search_index_docs = search_index_docs.replace("ENGINE", &engine.to_string());

        info!("search_index_docs   request URL  {:?}", &search_index_docs);

        let search_index_request = SearchMovieIndexRequest {
            q: q.to_string(),
            offset,
            limit,
        };

        info!("search_index_docs   request   {:?}", &search_index_request);

        let response = CLIENT
            .post(search_index_docs)
            .header(HEADER_X_PROCESSED_BY, processed_by)
            .header(HEADER_X_UUID, uuid)
            .header(HEADER_X_INITIATED_BY, initiated_by)
            .json(&search_index_request)
            .send()
            .await;

        if response.is_err() {
            error!(
                "error from SearchIndexDoc Service  {:?}",
                response.err().unwrap()
            );
            return (None, processed_by.to_string());
        }

        info!("search_index_docs   CLIENT.POST returned a OK response");

        match response {
            Ok(res) => {
                let (processed_by_new, _, _) =
                    get_trace_infos(res.headers(), "search_index_docs".to_string());

                if res.status() == StatusCode::OK {
                    info!("search_index_docs   CLIENT.POST returned an OK status code ");
                    let res = res.json::<MovieSearchResult>().await;

                    match res {
                        Ok(search_result) => {
                            info!(
                                "search_index_docs   got a valid search result {:?}",
                                &search_result
                            );
                            (Some(search_result), processed_by_new)
                        }
                        Err(e) => {
                            error!("search_index_doc service returned an error {:?}", e);
                            (None, processed_by_new)
                        }
                    }
                } else {
                    error!(
                        "search_index_doc Service returned status {}   ",
                        res.status()
                    );
                    (None, processed_by_new)
                }
            }
            Err(e) => {
                error!("search_index_doc service returned an error {:?}", e);
                (None, processed_by.to_string())
            }
        }
    }
}
