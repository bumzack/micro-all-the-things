use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

use crate::CONFIG;

pub const TABLE_LOG_ENTRY: &str = "logging";

pub fn create_pool() -> Pool {
    let mut pg_config = tokio_postgres::Config::new();

    let user: String = CONFIG
        .get("loggingservice.db.user")
        .expect("expected loggingservice.db.user variable");
    let password: String = CONFIG
        .get("loggingservice.db.password")
        .expect("expected loggingservice.db.password variable");
    let host: String = CONFIG
        .get("loggingservice.db.host")
        .expect("expected loggingservice.db.host variable");
    let dbname: String = CONFIG
        .get("loggingservice.db.name")
        .expect("expected loggingservice.db.name variable");

    println!("user {user}, password {password}, host {host}, dbname {dbname}");
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
