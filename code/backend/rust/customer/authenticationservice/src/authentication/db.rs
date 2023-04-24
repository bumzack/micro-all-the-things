use warp::Rejection;

pub type Result<T> = std::result::Result<T, Rejection>;

pub mod db_logging {
    use deadpool_postgres::Pool;
    use log::{error, info};
    use warp::reject;

    use common::logging::logging::DivideByZero;
    use common::models::authentication::AuthenticationEntry;

    use crate::db::db::TABLE_AUTHENTICATION;

    pub async fn find_authentication(
        pool: Pool,
        customer_id: i32,
    ) -> super::Result<AuthenticationEntry> {
        let client = pool.get().await.unwrap();

        let query = format!(
            "SELECT * FROM {}  WHERE customer_id = {}  AND jwt IS NOT NULL ",
            TABLE_AUTHENTICATION, customer_id
        );

        info!("SELECT query  {}", &query);

        // TODO
        // oh boy, that's beyond ugly
        let data = client.query_one(&query, &[]).await.map_err(|e| {
            error!("error rejection {:?}", e);
            reject::custom(DivideByZero)
        })?;

        let entry = AuthenticationEntry::from(&data);
        Ok(entry)
    }

    pub async fn insert_authentication(
        pool: Pool,
        customer_id: i32,
        token: &String,
    ) -> super::Result<AuthenticationEntry> {
        let client = pool.get().await.unwrap();
        let query = format!(
            "INSERT INTO {} (customer_id, jwt, logged_in) VALUES ($1, $2, $3) RETURNING *",
            TABLE_AUTHENTICATION
        );

        let row = client
            .query_one(
                query.as_str(),
                &[&customer_id, &token, &chrono::offset::Utc::now()],
            )
            .await
            .map_err(|e| {
                error!(
                    "error inserting a new entry in table {} {:?}",
                    &TABLE_AUTHENTICATION, e
                );
                reject::custom(DivideByZero)
            })?;

        let entry = AuthenticationEntry::from(&row);
        Ok(entry)
    }

    pub async fn update_authentication_logout(
        pool: Pool,
        customer_id: i32,
    ) -> super::Result<AuthenticationEntry> {
        let client = pool.get().await.unwrap();
        let query = format!(
            "UPDATE {} SET jwt = NULL, logged_out = $1 WHERE customer_id = $2  RETURNING *",
            TABLE_AUTHENTICATION
        );

        let row = client
            .query_one(query.as_str(), &[&chrono::offset::Utc::now(), &customer_id])
            .await
            .map_err(|e| {
                error!(
                    "error updating an existing entry in table {} {:?}",
                    &TABLE_AUTHENTICATION, e
                );
                reject::custom(DivideByZero)
            })?;

        let entry = AuthenticationEntry::from(&row);
        Ok(entry)
    }

    pub async fn update_authentication_login(
        pool: Pool,
        jwt: &String,
        customer_id: i32,
    ) -> super::Result<AuthenticationEntry> {
        let client = pool.get().await.unwrap();
        let query = format!(
            "UPDATE {} SET jwt = $1, logged_in = $2, logged_out = NULL WHERE customer_id = $3  RETURNING *",
            TABLE_AUTHENTICATION
        );

        let row = client
            .query_one(
                query.as_str(),
                &[jwt, &chrono::offset::Utc::now(), &customer_id],
            )
            .await
            .map_err(|e| {
                error!(
                    "error updating an existing entry in table {} {:?}",
                    &TABLE_AUTHENTICATION, e
                );
                reject::custom(DivideByZero)
            })?;

        let entry = AuthenticationEntry::from(&row);
        Ok(entry)
    }
}
