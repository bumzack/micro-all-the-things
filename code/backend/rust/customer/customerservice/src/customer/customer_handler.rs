pub mod handler_customer {
    use std::time::Instant;

    use deadpool_postgres::Pool;
    use log::{error, info};
    use reqwest::header::HeaderMap;
    use serde_json::json;
    use warp::{reject, Rejection, Reply};

    use common::entity::entity::Engine;
    use common::logging::logging::DivideByZero;
    use common::logging::tracing_headers::tracing_headers_stuff::{
        build_response_from_json, build_tracing_headers, get_trace_infos,
    };
    use common::models::customer::AddCustomer;
    use common::models::person::Person;
    use common::models::search_doc::SearchPaginatedRequest;

    use crate::customer::db::db_customer::{
        get_customer, get_customers_paginated, insert_customer,
    };
    use crate::{CLIENT, CONFIG};

    const SERVICE_NAME: &str = "Customer Service";

    pub async fn insert_customer_handler(
        pool: Pool,
        req: AddCustomer,
        headers: HeaderMap,
    ) -> Result<impl Reply, Rejection> {
        let start_total = Instant::now();

        let (initiated_by, uuid, processed_by) =
            get_trace_infos(&headers, SERVICE_NAME.to_string());

        info!("adding customer entry {:?}", req);

        let customer = insert_customer(pool.clone(), req)
            .await
            // TODO fix CustomError
            .map_err(|e| {
                error!("error rejection {:?}", e);
                reject::custom(DivideByZero)
            })?;

        let msg = format!("inserted new customer with id  {} ", customer.id);

        let headers = build_tracing_headers(
            &start_total,
            &SERVICE_NAME.to_string(),
            &initiated_by,
            &uuid,
            &processed_by,
            &msg,
        );

        let response = build_response_from_json(customer, headers);

        Ok(response)
    }

    pub async fn read_customer_paginated_handler(
        pool: Pool,
        offset: i32,
        limit: i32,
        headers: HeaderMap,
    ) -> Result<impl Reply, Rejection> {
        let start_total = Instant::now();

        let (initiated_by, uuid, processed_by) =
            get_trace_infos(&headers, SERVICE_NAME.to_string());

        info!(
            "reading customers paginated   offset {}, limit {}",
            offset, limit
        );

        let customers = get_customers_paginated(pool.clone(), offset, limit)
            .await
            // TODO fix CustomError
            .map_err(|e| {
                error!("error rejection {:?}", e);
                reject::custom(DivideByZero)
            })?;

        let msg = format!("reading {} customers paginated", customers.len());

        let headers = build_tracing_headers(
            &start_total,
            &SERVICE_NAME.to_string(),
            &initiated_by,
            &uuid,
            &processed_by,
            &msg,
        );

        let response = build_response_from_json(customers, headers);

        Ok(response)
    }

    pub async fn read_customer_handler(
        pool: Pool,
        email: String,
        headers: HeaderMap,
    ) -> Result<impl Reply, Rejection> {
        let start_total = Instant::now();

        let (initiated_by, uuid, processed_by) =
            get_trace_infos(&headers, SERVICE_NAME.to_string());

        info!("reading customer for email {:?}", &email);

        let customer = get_customer(pool, email)
            .await
            // TODO fix CustomError
            .map_err(|e| {
                error!("error rejection {:?}", e);
                reject::custom(DivideByZero)
            })?;

        let msg = format!("found customer id {}", customer.id);

        let headers = build_tracing_headers(
            &start_total,
            &SERVICE_NAME.to_string(),
            &initiated_by,
            &uuid,
            &processed_by,
            &msg,
        );

        let response = build_response_from_json(customer, headers);

        Ok(response)
    }

    pub async fn insert_dummy_data_handler(
        mut offset: u32,
        limit: u32,
        count: u32,
        pool: Pool,
        headers: HeaderMap,
    ) -> Result<impl Reply, Rejection> {
        info!("inserting all persons as customers");

        let start_total = Instant::now();

        let (initiated_by, uuid, processed_by) =
            get_trace_infos(&headers, SERVICE_NAME.to_string());

        let mut persons_found = true;
        let mut persons_processed = 0;
        while persons_found && persons_processed < count {
            let persons = search_persons(limit, offset, Engine::Solr).await;
            persons_found = !persons.is_empty();
            for p in persons {
                let email = format!("{}@foryouandyourfakewebshop.at", p.nconst);
                let (first_name, last_name) = match p.primary_name {
                    Some(n) => {
                        let mut names: Vec<&str> = n.split_ascii_whitespace().collect();
                        if names.len() > 1 {
                            let first_name = names.get(0).unwrap().to_string();
                            let last_names: Vec<&str> = names.drain(1..).collect();
                            let last_name = last_names.join(" / ");
                            (first_name, last_name)
                        } else {
                            // oh boy
                            let n = names.get(0).unwrap().to_string();
                            (n.clone(), n)
                        }
                    }
                    None => (p.nconst.clone(), p.nconst),
                };
                let add_customer = AddCustomer {
                    first_name,
                    last_name,
                    email,
                    password: "1234".to_string(),
                };
                let _ = insert_customer(pool.clone(), add_customer).await;
                persons_processed += 1;
            }
            offset += limit;
        }
        let msg = format!("customers processed {}", persons_processed);

        let headers = build_tracing_headers(
            &start_total,
            &SERVICE_NAME.to_string(),
            &initiated_by,
            &uuid,
            &processed_by,
            &msg,
        );

        let response = build_response_from_json(msg, headers);

        Ok(response)
    }

    async fn search_persons(limit: u32, offset: u32, engine: Engine) -> Vec<Person> {
        info!("rust_customerservice_insert_dummy_data. search_persons");
        let search_person: String = CONFIG
            .get("search_person_doc")
            .expect("expected search_person_doc POST request URL");

        let search_person = search_person.replace("ENGINE", &engine.to_string());

        let search_request = SearchPaginatedRequest {
            q: "*".to_string(),
            offset,
            limit,
            sort: vec!["nconst:asc".to_string()],
            next_cursor_mark: None,
        };

        let message = format!(
            "start rust_customerservice_insert_dummy_data. search_persons().  offset {}, limit {}, sort {:?}, engine: {}",
            offset,
            limit,
            &search_request.sort.clone(),
            engine.to_string()
        );
        info!("message {}", &message);

        info!("search person POST URL {}", &search_person);
        let json = json!(&search_request);
        let response = CLIENT.post(search_person).json(&json).send().await;

        let _message = format!(
            "error rust_customerservice_insert_dummy_data. search_persons(). offset {}, limit {}, sort {:?}.",
            offset,
            limit,
            &search_request.sort.clone()
        );
        let _msg = "search for persons paginated search request".to_string();

        if response.is_err() {
            error!(
                "error from SearchMovie Service {:?}",
                response.err().unwrap()
            );
            return vec![];
        }
        info!("XXX     search_persons all good");

        let response2 = response.unwrap();
        let persons = response2
            .json::<Vec<Person>>()
            .await
            .expect("expected a list of persons");
        info!(
            "rust_customerservice_insert_dummy_data. search_persons all good. found {} persons",
            persons.len()
        );

        let message = format!(
            "end rust_customerservice_insert_dummy_data. search_persons().  offset {}, limit {}, sort {:?}. {} persons found ",
            offset,
            limit,
            &search_request.sort.clone(),
            persons.len()
        );
        info!("message {}", &message);

        info!("rust_customerservice_insert_dummy_data search_persons finished successfully");

        persons
    }
}
