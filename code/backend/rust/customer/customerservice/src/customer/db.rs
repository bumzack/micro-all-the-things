use warp::Rejection;

pub type Result<T> = std::result::Result<T, Rejection>;

pub mod db_logging {
    use deadpool_postgres::Pool;
    use warp::reject;

    use common::logging::logging::DivideByZero;
    use common::models::customer::{AddCustomer, Customer};

    use crate::db::db::TABLE_CUSTOMER;

    pub async fn get_customer(pool: Pool, email: String) -> super::Result<Customer> {
        let client = pool.get().await.unwrap();

        let query = format!(
            "SELECT * FROM {}  WHERE email = '{}' ",
            TABLE_CUSTOMER, email
        );

        info!("SELECT query  {}", &query);

        // TODO
        //  oh boy, that's beyond ugly
        let mut data = client.query_one(&query, &[]).await.map_err(|e| {
            error!("error rejection {:?}", e);
            reject::custom(DivideByZero)
        })?;

        let entry = Customer::from(&data);
        Ok(entry)
    }

    pub async fn get_customers_paginated(
        pool: Pool,
        offset: i32,
        limit: i32,
    ) -> super::Result<Vec<Customer>> {
        let client = pool.get().await.unwrap();

        let query = format!(
            "SELECT * FROM  {}  ORDER BY id  LIMIT {} OFFSET {} ",
            TABLE_CUSTOMER, limit, offset
        );

        info!("SELECT customers paginated query  {}", &query);

        // TODO
        //  oh boy, that's beyond ugly
        let data = client.query(&query, &[]).await.map_err(|e| {
            error!("error rejection {:?}", e);
            reject::custom(DivideByZero)
        })?;
        if data.is_empty() {
            return Err(reject::not_found());
        }
        let customers: Vec<Customer> = data.iter().map(Customer::from).collect();
        Ok(customers)
    }

    pub async fn insert_customer(pool: Pool, customer: AddCustomer) -> super::Result<Customer> {
        let client = pool.get().await.unwrap();
        let query = format!(
            "INSERT INTO {} (first_name, last_name, email, password) VALUES ($1, $2, $3, $4) RETURNING *",
            TABLE_CUSTOMER
        );

        let row = client
            .query_one(
                query.as_str(),
                &[
                    &customer.first_name,
                    &customer.last_name,
                    &customer.email,
                    &customer.password,
                ],
            )
            .await
            .map_err(|e| {
                error!("error rejection {:?}", e);
                reject::custom(DivideByZero)
            })?;

        let entry = Customer::from(&row);
        Ok(entry)
    }
}
