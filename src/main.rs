mod db;

extern crate openssl;

#[macro_use]
extern crate diesel;

mod api;

use env_logger;

use dotenv::dotenv;
use crate::db::dbconn;
use crate::api::buildings_api::*;
use crate::api::rooms_api::*;
use crate::api::storeys_api::*;
use actix_web::{App, HttpServer};
use log::info;

use std::io::{ErrorKind, Error};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    info!("initializing logging...");
    dotenv().ok();
    env_logger::init();
    
    info!("attempting to connect to database service...");
    if dbconn::init().is_err() { return Err(Error::new(ErrorKind::Other, "could not connect to DB service")); }
    info!("database connection successful");

    info!("starting API service");
    HttpServer::new(|| {
        App::new()
            .service(get_all_buildings)
            .service(add_building)
            .service(get_building_by_id)
            .service(update_building)
            .service(delete_building)
            .service(get_all_storeys)
            .service(add_storey)
            .service(get_storey_by_id)
            .service(update_storey)
            .service(delete_storey)
            .service(get_all_rooms)
            .service(add_room)
            .service(get_room_by_id)
            .service(update_room)
            .service(delete_room)
    }).bind(("0.0.0.0", 9000))?.run().await

}
