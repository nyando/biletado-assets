extern crate dotenv;

use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use std::env;

pub fn db_connect() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}
