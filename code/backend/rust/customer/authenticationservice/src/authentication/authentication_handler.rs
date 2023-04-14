pub mod handler_authentication {
    use std::collections::BTreeMap;

    use deadpool_postgres::Pool;
    use hmac::{Hmac, Mac};
    use jwt::{AlgorithmType, Header, SignWithKey, Token};
    use sha2::Sha384;
    use warp::{reject, Rejection, reply, Reply};
    use warp::http::StatusCode;
    use warp::reply::json;

    use common::logging::logging::DivideByZero;
    use common::logging::logging_service_client::logging_service;
    use common::models::authentication::{LogInRequest, LogOutRequest};
    use common::models::customer::Customer;

    use crate::{CLIENT, CONFIG};
    use crate::authentication::db::db_logging::{
        find_authentication, insert_authentication, update_authentication_login,
        update_authentication_logout,
    };

    pub async fn check_authenticated_handler(
        pool: Pool,
        customer_id: i32,
    ) -> Result<impl Reply, Rejection> {
        info!(
            "find authentication entry for customer_id: {:?}",
            customer_id
        );

        let authentication = find_authentication(pool.clone(), customer_id)
            .await
            // TODO fix CustomError
            .map_err(|e| {
                info!("customer is not logged in  {:?}", e);
                reject::not_found()
            })?;

        Ok(json(&authentication))
    }

    // pub async fn read_authentication_handler(
    //     pool: Pool,
    //     email: String,
    // ) -> Result<impl Reply, Rejection> {
    //     info!("reading authentication for email entries {:?}", &email);
    //
    //     let customer = get_customer(pool, email)
    //         .await
    //         // TODO fix CustomError
    //         .map_err(|e| {
    //             error!("error rejection {:?}", e);
    //             reject::custom(DivideByZero)
    //         })?;
    //
    //     Ok(json(&customer))
    // }

    pub async fn login_handler(
        pool: Pool,
        login_request: LogInRequest,
    ) -> Result<impl Reply, Rejection> {
        info!(
            "trying to login customers paginated   login_request {:?}",
            login_request,
        );

        let customer = search_customer(&login_request.email).await;

        match &customer {
            Some(c) => {
                let auth = find_authentication(pool.clone(), c.id).await;
                match auth {
                    Ok(a) => {
                        if a.jwt.is_some() {
                            return Ok(reply::with_status(json(&a), StatusCode::OK));
                        }
                        if c.password.eq(&login_request.password)
                            && c.email.eq(&login_request.email)
                        {
                            let token = get_token(c);
                            info!("customer token {}", &token);
                            let aa = update_authentication_login(pool.clone(), &token, c.id).await;
                            let aa = aa.expect("expect db update to be successful");
                            return Ok(reply::with_status(json(&aa), StatusCode::OK));
                        }
                    }
                    Err(e) => {
                        info!("this is an error but shouldn't be {:?}", e);
                        if c.password.eq(&login_request.password)
                            && c.email.eq(&login_request.email)
                        {
                            let token = get_token(c);
                            info!("customer token {}", &token);

                            let res = insert_authentication(pool.clone(), c.id, &token).await;
                            return match res {
                                Ok(authentication_entry) => Ok(reply::with_status(
                                    json(&authentication_entry),
                                    StatusCode::OK,
                                )),
                                Err(e) => Ok(reply::with_status(
                                    json(&format!("{:?}", e)),
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                )),
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
    ) -> Result<impl Reply, Rejection> {
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
                    Ok(aa) => Ok(reply::with_status(json(&aa), StatusCode::OK)),
                    Err(e) => {
                        info!("error updating authentication entry {:?}", e);
                        Ok(reply::with_status(
                            json(&format!("{:?}", e)),
                            StatusCode::INTERNAL_SERVER_ERROR,
                        ))
                    }
                }
            }
            Err(e) => {
                info!("error cant find customer entry {:?}", e);
                Err(reject::not_found())
            }
        }
    }

    async fn search_customer(email: &String) -> Option<Customer> {
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
        logging_service::log_entry(" search_customer".to_string(), "INFO".to_string(), &message)
            .await;

        let response = CLIENT.get(search_customer).send().await;

        if response.is_err() {
            error!("error from CustomerService {:?}", response.err().unwrap());
            return None;
        }
        info!("search_customer all good");

        match response {
            Ok(r) => {
                if r.status().as_u16() < 299 {
                    let customer = r.json::<Customer>().await.expect("expected a customer");
                    return Some(customer);
                } else {
                    info!("error retrieving customer {}", r.status());
                    info!("error retrieving {:?}", r.text().await.unwrap());
                }
                None
            }
            Err(e) => {
                error!("got an error  {:?}", e);
                None
            }
        }
    }
}
