pub mod search_helper {
    use log::error;
    use reqwest::StatusCode;

    use common::models::customer_prices::CustomerPriceEntry;
    use common::models::prices::PriceEntry;

    use crate::{CLIENT, CONFIG};

    pub async fn get_movie_price(tconst: &String) -> Option<PriceEntry> {
        let search_price: String = CONFIG
            .get("search_movie_price")
            .expect("expected search_movie_price GET request URL");

        let search_price = search_price.replace(":tconst", &tconst.to_string());
        let response = CLIENT.get(search_price).send().await;

        if response.is_err() {
            error!("error from PriceService {:?}", response.err().unwrap());
            return None;
        }

        match response {
            Ok(res) => {
                if res.status() == StatusCode::OK {
                    let price_entry = res.json::<PriceEntry>().await;

                    match price_entry {
                        Ok(price_entry) => Some(price_entry),
                        Err(e) => {
                            error!("priceservice returned an error {:?}", e);
                            None
                        }
                    }
                } else {
                    error!(
                        "priceservice returned status {} for tconst {}. ",
                        res.status(),
                        tconst
                    );
                    None
                }
            }
            Err(e) => {
                error!("priceservice service returned an error {:?}", e);
                None
            }
        }
    }

    pub async fn get_movie_customerprice(year: i32, id: i32) -> Option<CustomerPriceEntry> {
        let search_customerprice: String = CONFIG
            .get("search_movie_customerprice")
            .expect("expected search_movie_customerprice GET request URL");

        let search_customerprice = search_customerprice.replace(":customer_id", &id.to_string());
        let search_customerprice = search_customerprice.replace(":year", &year.to_string());
        let response = CLIENT.get(search_customerprice).send().await;

        if response.is_err() {
            error!(
                "error from CustomerPriceService {:?}",
                response.err().unwrap()
            );
            return None;
        }

        match response {
            Ok(res) => {
                if res.status() == StatusCode::OK {
                    let price_entry = res.json::<CustomerPriceEntry>().await;

                    match price_entry {
                        Ok(price_entry) => Some(price_entry),
                        Err(e) => {
                            error!("CustomerPriceService returned an error {:?}", e);
                            None
                        }
                    }
                } else {
                    error!(
                        "CustomerPriceService returned status {} for customer_id {}, year {}. ",
                        res.status(),
                        &id,
                        &year,
                    );
                    None
                }
            }
            Err(e) => {
                error!("CustomerPriceService service returned an error {:?}", e);
                None
            }
        }
    }
}
