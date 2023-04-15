pub mod mod_search_helper {
    use common::entity::entity::Engine;
    use log::error;
    use reqwest::StatusCode;

    use common::models::article::SearchCustomer;
    use common::models::authentication::AuthenticationEntry;
    use common::models::search_doc::{MovieSearchResult, SearchMovieIndexRequest};

    use crate::{CLIENT, CONFIG};

    pub async fn get_authentication_entry(
        search_customer: &SearchCustomer,
    ) -> Option<AuthenticationEntry> {
        let search_auth: String = CONFIG
            .get("customer_authenticated")
            .expect("expected customer_authenticated GET request URL");

        match search_customer.customer_id {
            Some(id) => {
                let search_auth = search_auth.replace(":customer_id", &id.to_string());
                let response = CLIENT.get(search_auth).send().await;

                if response.is_err() {
                    error!(
                        "error from AuthenticatioService {:?}",
                        response.err().unwrap()
                    );
                    return None;
                }

                match response {
                    Ok(res) => {
                        if res.status() == StatusCode::OK {
                            let auth = res.json::<AuthenticationEntry>().await;

                            match auth {
                                Ok(auth) => {
                                    if auth.jwt.is_some()
                                        && auth.logged_in.is_some()
                                        && auth.logged_out.is_none()
                                    {
                                        Some(auth)
                                    } else {
                                        None
                                    }
                                }
                                Err(e) => {
                                    error!("authentication service returned an error {:?}", e);
                                    None
                                }
                            }
                        } else {
                            error!("authentication Service returned status {} for customer.id {}. assuming not logged in. ", res.status(), id);
                            None
                        }
                    }
                    Err(e) => {
                        error!("authentication service returned an error {:?}", e);
                        None
                    }
                }
            }

            None => {
                error!("no customer id provided ");
                None
            }
        }
    }

    pub async fn search_index_docs(
        engine: Engine,
        q: &String,
        limit: u32,
        offset: u32,
    ) -> Option<MovieSearchResult> {
        let search_index_docs: String = CONFIG
            .get("search_index_doc")
            .expect("expected search_index_doc GET request URL");

        let search_index_docs = search_index_docs.replace("ENGINE", &engine.to_string());

        let search_index_request = SearchMovieIndexRequest {
            q: q.to_string(),
            offset,
            limit,
        };

        let response = CLIENT
            .post(search_index_docs)
            .json(&search_index_request)
            .send()
            .await;

        if response.is_err() {
            error!(
                "error from SearchIndexDoc Service  {:?}",
                response.err().unwrap()
            );
            return None;
        }

        match response {
            Ok(res) => {
                if res.status() == StatusCode::OK {
                    let res = res.json::<MovieSearchResult>().await;

                    match res {
                        Ok(search_result) => Some(search_result),
                        Err(e) => {
                            error!("search_index_doc service returned an error {:?}", e);
                            None
                        }
                    }
                } else {
                    error!(
                        "search_index_doc Service returned status {}   ",
                        res.status()
                    );
                    None
                }
            }
            Err(e) => {
                error!("search_index_doc service returned an error {:?}", e);
                None
            }
        }
    }
}
