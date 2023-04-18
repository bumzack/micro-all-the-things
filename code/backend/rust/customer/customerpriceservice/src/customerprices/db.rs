use warp::Rejection;

pub type Result<T> = std::result::Result<T, Rejection>;

pub mod db_logging {
    use deadpool_postgres::Pool;
    use warp::reject;

    use common::logging::logging::DivideByZero;
    use common::models::customer_prices::{AddCustomerPriceEntry, CustomerPriceEntry};

    use crate::db::db::TABLE_CUSTOMER_PRICE;

    pub async fn get_customerprice(
        pool: Pool,
        customer_id: &String,
        year: i32,
    ) -> super::Result<CustomerPriceEntry> {
        let client = pool.get().await.unwrap();

        let query = format!(
            "SELECT * FROM {}  WHERE customer_id = {}  AND start_year <= {} AND {} <= end_year ",
            TABLE_CUSTOMER_PRICE, customer_id, year, year
        );

        info!("SELECT query  {}", &query);

        // TODO
        //  oh boy, that's beyond ugly
        let entry = client.query_one(&query, &[]).await.map_err(|e| {
            error!("error rejection {:?}", e);
            reject::not_found()
        })?;

        let entry = CustomerPriceEntry::from(&entry);

        Ok(entry)
    }

    pub async fn insert_price_entry(
        pool: Pool,
        req: AddCustomerPriceEntry,
    ) -> super::Result<CustomerPriceEntry> {
        let client = pool.get().await.unwrap();
        let query = format!(
            "INSERT INTO {} (customer_id, discount, start_year, end_year) VALUES ($1, $2, $3, $4) RETURNING *",
            TABLE_CUSTOMER_PRICE
        );

        let row = client
            .query_one(
                query.as_str(),
                &[
                    &req.customer_id,
                    &req.discount,
                    &req.start_year,
                    &req.end_year,
                ],
            )
            .await
            .map_err(|e| {
                error!("error rejection {:?}", e);
                reject::custom(DivideByZero)
            })?;
        let entry = CustomerPriceEntry::from(&row);
        Ok(entry)
    }
}
