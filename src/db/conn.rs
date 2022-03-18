extern crate dotenv;

use crate::db::schema::buildings::dsl::buildings;
use crate::db::models::Building;

use diesel::{Connection, PgConnection, RunQueryDsl};
use dotenv::dotenv;
use std::env;

pub fn db_connect() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

pub fn get_buildings() -> Vec<Building> {
    let conn = db_connect();
    buildings.load::<Building>(&conn).expect("Error loading buildings")
}