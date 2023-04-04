use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

use crate::CONFIG;

pub const TABLE_LOG_ENTRY: &str = "logging";

pub fn create_pool() -> Pool {
    let mut pg_config = tokio_postgres::Config::new();

    let user: String = CONFIG
        .get("loggingservice_db_user")
        .expect("expected loggingservice_db_user variable");
    let password: String = CONFIG
        .get("loggingservice_db_password")
        .expect("expected loggingservice_db_password variable");
    let host: String = CONFIG
        .get("loggingservice_db_host")
        .expect("expected loggingservice_db_host variable");
    let dbname: String = CONFIG
        .get("loggingservice_db_name")
        .expect("expected loggingservice_db_name variable");

    info!("user {user}, password {password}, host {host}, dbname {dbname}");
    pg_config.user(&user);
    pg_config.password(&password);
    pg_config.host(&host);
    pg_config.dbname(&dbname);
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    Pool::builder(mgr).max_size(16).build().unwrap()
}
