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

pub fn print_buildings() {
    let conn = db_connect();
    let results = buildings.load::<Building>(&conn).expect("Error loading buildings");
    println!("found {} buildings", results.len());
    for building in results {
        println!("name: {}", building.name);
    }
}