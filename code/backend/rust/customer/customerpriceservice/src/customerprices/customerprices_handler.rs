pub mod filters_customer_price {
    use deadpool_postgres::Pool;
    use reqwest::StatusCode;
    use warp::{reject, Rejection, Reply};
    use warp::reply::json;

    use common::logging::logging::DivideByZero;
    use common::logging::logging_service_client::logging_service;
    use common::models::customer::Customer;
    use common::models::customer_prices::AddCustomerPriceEntry;

    use crate::{CLIENT, CONFIG};
    use crate::customerprices::db::db_logging::{get_customerprice, insert_price_entry};

    pub async fn insert_customer_price_handler(
        pool: Pool,
        req: AddCustomerPriceEntry,
    ) -> Result<impl Reply, Rejection> {
        info!("adding customerprices entry {:?}", req);

        let customer_price = insert_price_entry(pool.clone(), req)
            .await
            // TODO fix CustomError
            .map_err(|e| {
                error!("error rejection {:?}", e);
                reject::custom(DivideByZero)
            })?;

        Ok(json(&customer_price))
    }

    pub async fn read_customerprice_entry(
        pool: Pool,
        customer_id: String,
        year: i32,
    ) -> Result<impl Reply, Rejection> {
        info!(
            "reading customerprices entries. customer_id  {:?}, year: {}",
            &customer_id, year,
        );

        let customer_price_entry = get_customerprice(pool, &customer_id, year)
            .await
            // TODO fix CustomError
            .map_err(|e| {
                error!("error rejection {:?}", e);
                reject::not_found()
            })?;

        info!(
            "found a customerprice for  customer_id {:?}, year {}:  {:?}",
            &customer_id, year, &customer_price_entry
        );
        Ok(json(&customer_price_entry))
    }

    pub async fn insert_dummy_data_customer_prices_handler(
        mut offset: u32,
        limit: u32,
        count: u32,
        pool: Pool,
    ) -> Result<impl Reply, Rejection> {
        info!("inserting dummy customer customer ");

        let years_ranges = vec![
            (0, 1900),
            (1901, 1910),
            (1911, 1920),
            (1921, 1930),
            (1931, 1940),
            (1941, 1950),
            (1951, 1960),
            (1961, 1970),
            (1971, 1980),
            (1981, 1990),
            (1991, 2000),
            (2001, 2010),
            (2011, 2020),
            (2021, 2030),
        ];

        let mut customers_found = true;
        let mut customers_processed = 0;
        let mut pricerows_inserted = 0;
        while customers_found && customers_processed < count {
            let customers = search_customers(limit, offset, count).await;
            customers_found = !customers.is_empty();
            for customer in customers {
                let years_ranges = years_ranges.clone();
                for (idx, year) in years_ranges.into_iter().enumerate() {
                    let discount = 3.0 * (idx as f32 + 1.0) + rand::random::<f32>() * 20.0;
                    let mut discount = (discount * 10.0).round() / 10.0;

                    if discount > 90.0 {
                        discount = 91.1;
                    }

                    let add_customer = AddCustomerPriceEntry {
                        customer_id: customer.id,
                        start_year: year.0,
                        end_year: year.1,
                        discount,
                    };
                    let _ = insert_price_entry(pool.clone(), add_customer).await;
                    pricerows_inserted += 1;
                }
                customers_processed += 1;
            }
            offset += limit;
        }
        let msg = format!(
            "customers processed {}. inserted  {} pricerows",
            customers_processed, pricerows_inserted
        );
        Ok(warp::reply::with_status(msg, StatusCode::CREATED))
    }

    async fn search_customers(limit: u32, offset: u32, count: u32) -> Vec<Customer> {
        info!("rust_customerpriceservice_insert_dummy_data. search_persons");
        let search_customer: String = CONFIG
            .get("search_customers_paginated")
            .expect("expected search_customers_paginated GET request URL");

        // search_customers_paginated = "http://localhost:18980/api/v1/customer/paginated/:offset/:limit/:count"
        let search_customer = search_customer.replace(":offset", &offset.to_string());
        let search_customer = search_customer.replace(":limit", &limit.to_string());
        //  let search_customer = search_customer.replace(":count", &count.to_string());

        let message = format!(
            "start rust_customerpriceservice_insert_dummy_data. search_customers().  offset {}, limit {}, count {:?}, url: {}",
            offset,
            limit,
            count,
            search_customer,
        );
        info!("message {}", &message);
        logging_service::log_entry(
            " rust_customerpriceservice_insert_dummy_data".to_string(),
            "INFO".to_string(),
            &message,
        )
            .await;

        let response = CLIENT.get(search_customer).send().await;

        if response.is_err() {
            error!("error from CustomerService {:?}", response.err().unwrap());
            return vec![];
        }
        info!("search_customers all good");

        let response2 = response.unwrap();
        let customers = response2
            .json::<Vec<Customer>>()
            .await
            .expect("expected a list of customers");
        info!(
            "rust_customerpriceservice_insert_dummy_data. search_customers all good. found {} customers",
            customers.len()
        );

        let message = format!(
            "end rust_customerpriceservice_insert_dummy_data. search_customers().  offset {}, limit {},  {} customers found ",
            offset,
            limit,
            customers.len()
        );
        info!("message {}", &message);
        logging_service::log_entry(
            " rust_customerpriceservice_insert_dummy_data".to_string(),
            "INFO".to_string(),
            &message,
        )
            .await;
        info!(
            ".rust_customerpriceservice_insert_dummy_data search_customers finished successfully"
        );

        customers
    }
}
