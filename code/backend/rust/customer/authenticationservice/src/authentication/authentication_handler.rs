pub mod handler_authentication {
    use deadpool_postgres::Pool;
    use warp::reply::json;
    use warp::{reject, Rejection, Reply};

    use common::logging::logging::DivideByZero;
    use common::logging::logging_service_client::logging_service;
    use common::models::authentication::{LogInRequest, LogOutRequest};
    use common::models::customer::Customer;

    use crate::authentication::db::db_logging::find_authentication;
    use crate::{CLIENT, CONFIG};

    pub async fn check_login_handler(
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
                error!("error rejection {:?}", e);
                reject::custom(DivideByZero)
            })?;

        Ok(json(&authentication))
    }

    pub async fn read_authentication_handler(
        pool: Pool,
        email: String,
    ) -> Result<impl Reply, Rejection> {
        info!("reading authentication for email entries {:?}", &email);

        let customer = get_customer(pool, email)
            .await
            // TODO fix CustomError
            .map_err(|e| {
                error!("error rejection {:?}", e);
                reject::custom(DivideByZero)
            })?;

        Ok(json(&customer))
    }

    pub async fn login_handler(
        pool: Pool,
        login_request: LogInRequest,
    ) -> Result<impl Reply, Rejection> {
        info!(
            "trying to login customers paginated   login_request {:?}",
            login_request,
        );

        let customer = search_customer(login_request.email).await;

        let customer = match customer {
            Some(c) => c,
            None => return Err(reject::not_found()),
        };

        Ok(json(&customer))
    }

    pub async fn logout_handler(
        pool: Pool,
        logout_request: LogOutRequest,
    ) -> Result<impl Reply, Rejection> {
        info!(
            "trying to logout customer     logout_request {:?}",
            logout_request,
        );

        let customer = find_authentication(pool, logout_request.customer_id).await;

        let customer = match customer {
            Some(c) => c,
            None => return Err(reject::not_found()),
        };

        Ok(json(&customer))
    }

    async fn search_customer(email: String) -> Option<Customer> {
        info!("rust_customerpriceservice_insert_dummy_data. search_persons");
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

        let response2 = response.unwrap();
        let customer = response2
            .json::<Customer>()
            .await
            .expect("expected a customer");
        info!("search_customer. search_customer all good. found a customer",);

        let message = format!("end search_customer. found a customer {:?}", &customer,);
        info!("message {}", &message);
        logging_service::log_entry(" search_customer".to_string(), "INFO".to_string(), &message)
            .await;
        info!(".search_customer search_customer finished successfully");

        Some(customer)
    }
}
