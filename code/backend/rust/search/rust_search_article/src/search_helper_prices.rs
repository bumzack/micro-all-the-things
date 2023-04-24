pub mod search_helper {
    use log::{error, info};
    use reqwest::StatusCode;

    use common::logging::tracing_headers::tracing_headers_stuff::{
        get_trace_infos, HEADER_X_INITIATED_BY, HEADER_X_PROCESSED_BY, HEADER_X_UUID,
    };
    use common::models::customer_prices::CustomerPriceEntry;
    use common::models::prices::PriceEntry;

    use crate::{CLIENT, CONFIG};
    use crate::search_article_routes::mod_search_article_routes::SERVICE_NAME;

    pub async fn get_movie_price(
        tconst: &String,
        initiated_by: &String,
        uuid: &String,
        processed_by: &String,
    ) -> (Option<PriceEntry>, String) {
        let search_price: String = CONFIG
            .get("search_movie_price")
            .expect("expected search_movie_price GET request URL");

        let search_price = search_price.replace(":tconst", &tconst.to_string());
        let response = CLIENT
            .get(search_price)
            .header(HEADER_X_PROCESSED_BY, processed_by)
            .header(HEADER_X_UUID, uuid)
            .header(HEADER_X_INITIATED_BY, initiated_by)
            .send()
            .await;

        if response.is_err() {
            error!("error from PriceService {:?}", response.err().unwrap());
            return (None, processed_by.to_string());
        }

        match response {
            Ok(res) => {
                let (_, _, processed_by_new) =
                    get_trace_infos(res.headers(), SERVICE_NAME.to_string());

                if res.status() == StatusCode::OK {
                    let price_entry = res.json::<PriceEntry>().await;

                    match price_entry {
                        Ok(price_entry) => (Some(price_entry), processed_by_new.to_string()),
                        Err(e) => {
                            error!("priceservice returned an error {:?}", e);
                            (None, processed_by_new.to_string())
                        }
                    }
                } else {
                    error!(
                        "priceservice returned status {} for tconst {}. ",
                        res.status(),
                        tconst
                    );
                    (None, processed_by_new.to_string())
                }
            }
            Err(e) => {
                error!("priceservice service returned an error {:?}", e);
                (None, processed_by.to_string())
            }
        }
    }

    pub async fn get_movie_customerprice(
        year: i32,
        id: i32,
        initiated_by: &String,
        uuid: &String,
        processed_by: &String,
    ) -> (Option<CustomerPriceEntry>, String) {
        let search_customerprice: String = CONFIG
            .get("search_movie_customerprice")
            .expect("expected search_movie_customerprice GET request URL");

        let search_customerprice = search_customerprice.replace(":customer_id", &id.to_string());
        let search_customerprice = search_customerprice.replace(":year", &year.to_string());

        info!(
            "CustomerPriceService request URL     {:?}",
            &search_customerprice
        );

        let response = CLIENT
            .get(search_customerprice)
            .header(HEADER_X_PROCESSED_BY, processed_by)
            .header(HEADER_X_UUID, uuid)
            .header(HEADER_X_INITIATED_BY, initiated_by)
            .send()
            .await;

        if response.is_err() {
            error!(
                "error from CustomerPriceService {:?}",
                response.err().unwrap()
            );
            return (None, processed_by.to_string());
        }

        match response {
            Ok(res) => {
                let (_, _, processed_by_new) =
                    get_trace_infos(res.headers(), SERVICE_NAME.to_string());

                info!("CustomerPriceService response is ok");
                if res.status() == StatusCode::OK {
                    let price_entry = res.json::<CustomerPriceEntry>().await;

                    match price_entry {
                        Ok(price_entry) => {
                            info!("CustomerPriceService got a price {:?}", &price_entry);
                            (Some(price_entry), processed_by_new.to_string())
                        }
                        Err(e) => {
                            error!("CustomerPriceService returned an error {:?}", e);
                            (None, processed_by_new.to_string())
                        }
                    }
                } else {
                    error!(
                        "CustomerPriceService returned status {} for customer_id {}, year {}. ",
                        res.status(),
                        &id,
                        &year,
                    );
                    (None, processed_by_new.to_string())
                }
            }
            Err(e) => {
                error!("CustomerPriceService service returned an error {:?}", e);
                (None, processed_by.to_string())
            }
        }
    }
}
