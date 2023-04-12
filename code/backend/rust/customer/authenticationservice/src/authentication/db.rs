use warp::Rejection;

pub type Result<T> = std::result::Result<T, Rejection>;

pub mod db_logging {
    use deadpool_postgres::Pool;
    use warp::reject;

    use crate::db::db::TABLE_AUTHENTICATION;
    use common::logging::logging::DivideByZero;
    use common::models::authentication::AuthenticationEntry;
    use common::models::customer::{AddCustomer, Customer};

    pub async fn find_authentication(
        pool: Pool,
        customer_id: i32,
    ) -> super::Result<AuthenticationEntry> {
        let client = pool.get().await.unwrap();

        let query = format!(
            "SELECT * FROM {}  WHERE customer_id = '{}' ",
            TABLE_AUTHENTICATION, customer_id
        );

        info!("SELECT query  {}", &query);

        // TODO
        //  oh boy, that's beyond ugly
        let data = client.query_one(&query, &[]).await.map_err(|e| {
            error!("error rejection {:?}", e);
            reject::custom(DivideByZero)
        })?;

        let entry = AuthenticationEntry::from(&data);
        Ok(entry)
    }

    pub async fn insert_authentication(
        pool: Pool,
        customer: AddCustomer,
    ) -> super::Result<Customer> {
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
