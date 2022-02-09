// dotenv
use dotenv::dotenv;
use std::env;
// DB
// use mysql::prelude::Queryable;
use mysql::{Opts, OptsBuilder};
use r2d2_mysql::MysqlConnectionManager;
use std::sync::Arc;
// use std::thread;

const DATABASE_POOL_SIZE: u32 = 4;

// Database Connectionを返す
pub fn database() -> r2d2::PooledConnection<r2d2_mysql::MysqlConnectionManager> {
    dotenv().ok();
    let db_url = format!(
        "mysql://{user}:{pass}@{host}:{port}/{name}",
        user = env::var("DATABASE_USER").expect("DATABASE_USER must be set"),
        pass = env::var("DATABASE_PASS").expect("DATABASE_PASS must be set"),
        host = env::var("DATABASE_HOST").expect("DATABASE_HOST must be set"),
        port = env::var("DATABASE_PORT").expect("DATABASE_PORT must be set"),
        name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set")
    );
    let opts = Opts::from_url(&db_url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let manager = MysqlConnectionManager::new(builder);
    let pool = Arc::new(
        r2d2::Pool::builder()
            .max_size(DATABASE_POOL_SIZE)
            .build(manager)
            .unwrap(),
    );
    let pool = pool.clone();
    let conn = pool.get().unwrap();
    return conn;
}
