pub mod handler_authentication {
    use std::collections::BTreeMap;
    use std::time::Instant;

    use deadpool_postgres::Pool;
    use hmac::{Hmac, Mac};
    use jwt::{AlgorithmType, Header, SignWithKey, Token};
    use log::{error, info};
    use reqwest::header::HeaderMap;
    use sha2::Sha384;
    use warp::http::StatusCode;
    use warp::{reject, Rejection, Reply};

    use common::logging::logging::DivideByZero;
    use common::logging::tracing_headers::tracing_headers_stuff::{
        build_response_from_json, build_response_from_json_with_status, build_tracing_headers,
        get_trace_infos, HEADER_X_INITIATED_BY, HEADER_X_PROCESSED_BY, HEADER_X_UUID,
    };
    use common::models::authentication::{LogInRequest, LogOutRequest};
    use common::models::customer::Customer;

    use crate::authentication::db::db_logging::{
        find_authentication, insert_authentication, update_authentication_login,
        update_authentication_logout,
    };
    use crate::{CLIENT, CONFIG};

    const SERVICE_NAME: &str = "authenticated_service";

    pub async fn check_authenticated_handler(
        pool: Pool,
        customer_id: i32,
        map: HeaderMap,
    ) -> Result<impl Reply, Rejection> {
        let start_total = Instant::now();
        let (initiated_by, uuid, processed_by) = get_trace_infos(&map, SERVICE_NAME.to_string());

        info!(
            "uuid {} find authentication entry for customer_id: {:?}",
            &uuid, customer_id
        );

        let authentication = find_authentication(pool.clone(), customer_id)
            .await
            // TODO fix CustomError
            .map_err(|e| {
                info!("customer is not logged in  {:?}", e);
                reject::not_found()
            })?;

        let msg = format!("found auth for custoomer_id: {}", customer_id);
        let headers = build_tracing_headers(
            &start_total,
            &SERVICE_NAME.to_string(),
            &initiated_by,
            &uuid,
            &processed_by,
            &msg,
        );

        let response = build_response_from_json(authentication, headers);

        Ok(response)
    }

    pub async fn login_handler(
        pool: Pool,
        login_request: LogInRequest,
        map: HeaderMap,
    ) -> Result<impl Reply, Rejection> {
        let start_total = Instant::now();
        info!(
            "trying to login customers paginated   login_request {:?}",
            login_request,
        );

        let (initiated_by, uuid, processed_by) = get_trace_infos(&map, SERVICE_NAME.to_string());
        info!("initial  processed by  {:?}", &processed_by,);
        let (customer, processed_by_new) =
            search_customer(&login_request.email, &initiated_by, &uuid, &processed_by).await;

        match &customer {
            Some(c) => {
                let auth = find_authentication(pool.clone(), c.id).await;
                match auth {
                    Ok(a) => {
                        if a.jwt.is_some() {
                            let msg =
                                format!("user already logged in. email: {}", login_request.email);
                            let headers = build_tracing_headers(
                                &start_total,
                                &SERVICE_NAME.to_string(),
                                &initiated_by,
                                &uuid,
                                &processed_by_new,
                                &msg,
                            );

                            let response = build_response_from_json(a, headers);

                            return Ok(response);
                        }
                        if c.password.eq(&login_request.password)
                            && c.email.eq(&login_request.email)
                        {
                            let token = get_token(c);
                            info!("customer token {}", &token);
                            let aa = update_authentication_login(pool.clone(), &token, c.id).await;
                            let aa = aa.expect("expect db update to be successful");

                            let msg = format!(
                                "user succesfully logged in. email: {}",
                                login_request.email
                            );
                            let headers = build_tracing_headers(
                                &start_total,
                                &SERVICE_NAME.to_string(),
                                &initiated_by,
                                &uuid,
                                &processed_by_new,
                                &msg,
                            );

                            let response = build_response_from_json(aa, headers);

                            return Ok(response);
                        }
                    }
                    Err(e) => {
                        // WTF?
                        // whats going on here
                        info!("this is an error but shouldn't be. this is if no entry was found for the email/customer id {:?}", e);
                        if c.password.eq(&login_request.password)
                            && c.email.eq(&login_request.email)
                        {
                            let token = get_token(c);
                            info!("customer token {}", &token);

                            let res = insert_authentication(pool.clone(), c.id, &token).await;
                            return match res {
                                Ok(authentication_entry) => {
                                    let msg = format!(
                                        "new authentication entry inserted into DB for email: {} ",
                                        &login_request.email
                                    );
                                    let headers = build_tracing_headers(
                                        &start_total,
                                        &SERVICE_NAME.to_string(),
                                        &initiated_by,
                                        &uuid,
                                        &processed_by_new,
                                        &msg,
                                    );

                                    let response =
                                        build_response_from_json(authentication_entry, headers);

                                    Ok(response)
                                }

                                Err(e) => {
                                    error!("error doing stuff {:?}", e);
                                    let msg = "error authenticating user".to_string();
                                    let headers = build_tracing_headers(
                                        &start_total,
                                        &SERVICE_NAME.to_string(),
                                        &initiated_by,
                                        &uuid,
                                        &processed_by_new,
                                        &msg,
                                    );

                                    let response = build_response_from_json_with_status(
                                        msg,
                                        headers,
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                    );

                                    Ok(response)
                                }
                            };
                        }
                    }
                }
            }
            None => return Err(reject::not_found()),
        };

        Err(reject::not_found())
    }

    fn get_token(c: &Customer) -> String {
        let key: Hmac<Sha384> = Hmac::new_from_slice(b"this-is-a-really-secret-secret-password")
            .map_err(|e| {
                error!(
                    "error rejection  cant create key from slice invalid length{:?}",
                    e
                );
                reject::custom(DivideByZero)
            })
            .expect("dont know what to do - new_from_slice returns an error");
        let header = Header {
            algorithm: AlgorithmType::Hs384,
            ..Default::default()
        };
        let mut claims = BTreeMap::new();
        claims.insert("customer_id", c.id.to_string());
        let token = Token::new(header, claims)
            .sign_with_key(&key)
            .map_err(|e| {
                error!("error rejection when signing the JWT {:?}", e);
                reject::custom(DivideByZero)
            })
            .expect("sign_with_key returns an error");
        let token = token.as_str().to_string();
        info!("customer token {}", &token);
        token
    }

    pub async fn logout_handler(
        pool: Pool,
        logout_request: LogOutRequest,
        map: HeaderMap,
    ) -> Result<impl Reply, Rejection> {
        let start_total = Instant::now();
        let (initiated_by, uuid, processed_by) = get_trace_infos(&map, SERVICE_NAME.to_string());

        info!(
            "trying to logout customer     logout_request {:?}",
            logout_request,
        );

        let customer = find_authentication(pool.clone(), logout_request.customer_id).await;

        match customer {
            Ok(a) => {
                let authentication =
                    update_authentication_logout(pool.clone(), a.customer_id).await;
                match authentication {
                    Ok(aa) => {
                        let msg = "logout user success".to_string();
                        let headers = build_tracing_headers(
                            &start_total,
                            &SERVICE_NAME.to_string(),
                            &initiated_by,
                            &uuid,
                            &processed_by,
                            &msg,
                        );

                        let response = build_response_from_json(aa, headers);

                        Ok(response)
                    }
                    Err(e) => {
                        info!("error updating authentication entry {:?}", e);
                        let msg = "error authenticating user".to_string();
                        let headers = build_tracing_headers(
                            &start_total,
                            &SERVICE_NAME.to_string(),
                            &initiated_by,
                            &uuid,
                            &processed_by,
                            &msg,
                        );

                        let response = build_response_from_json_with_status(
                            msg,
                            headers,
                            StatusCode::INTERNAL_SERVER_ERROR,
                        );

                        Ok(response)
                    }
                }
            }
            Err(e) => {
                info!("error can't find customer entry {:?}", e);
                Err(reject::not_found())
            }
        }
    }

    async fn search_customer(
        email: &String,
        initiated_by: &String,
        uuid: &String,
        processed_by: &String,
    ) -> (Option<Customer>, String) {
        info!("rust_authenticationservice. search_customer");
        let search_customer: String = CONFIG
            .get("search_customer_by_email")
            .expect("expected search_customer_by_email GET request URL");

        let search_customer = search_customer.replace(":email", &email);

        let message = format!(
            "start search_customer. search_customer().  email {},   url: {}",
            &email, &search_customer,
        );
        info!("message {}", &message);

        let response = CLIENT
            .get(search_customer)
            .header(HEADER_X_PROCESSED_BY, processed_by)
            .header(HEADER_X_UUID, uuid)
            .header(HEADER_X_INITIATED_BY, initiated_by)
            .send()
            .await;

        if response.is_err() {
            error!("error from CustomerService {:?}", response.err().unwrap());
            return (None, processed_by.to_string());
        }
        info!("search_customer all good");

        match response {
            Ok(r) => {
                let (_, _, processed_by_new) =
                    get_trace_infos(r.headers(), "search_index_docs".to_string());
                info!("processed_by_new   {:?}", &processed_by,);
                if r.status().as_u16() < 299 {
                    let customer = r.json::<Customer>().await.expect("expected a customer");
                    return (Some(customer), processed_by_new);
                } else {
                    info!("error retrieving customer {}", r.status());
                    info!("error retrieving {:?}", r.text().await.unwrap());
                }
                (None, processed_by_new)
            }
            Err(e) => {
                error!("got an error  {:?}", e);
                (None, "something gone wrong".to_string())
            }
        }
    }
}
