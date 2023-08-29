use std::env;

use dotenvy::dotenv;
use mysql::prelude::*;

fn main() -> Result<(), mysql::Error> {
    dotenv().ok();

    let db_user = env::var("DB_USER").expect("DB_USER not set");
    let db_pass = env::var("DB_PASS").expect("DB_PASS not set");

    let mut opts = mysql::OptsBuilder::new();
    opts = opts.user(Some(db_user)).pass(Some(db_pass));

    let pool = mysql::Pool::new(opts)?;

    let mut conn = pool.get_conn()?;

    conn.query_drop("DROP DATABASE IF EXISTS scraping")?;
    conn.query_drop("CREATE DATABASE scraping")?;
    conn.query_drop("USE scraping")?;

    conn.query_drop(
        r#"
        CREATE TABLE pages (
            id BIGINT NOT NULL AUTO_INCREMENT,
            title VARCHAR(200),
            content VARCHAR(10000),
            created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY(id)
        )
    "#,
    )?;

    Ok(())
}
