// DO NOT CHANGE THE ORDER OF THESE TWO IMPORTS ON PENALTY OF DEATH
extern crate openssl;
#[macro_use]
extern crate diesel;
// THE DOCKER BUILD BREAKS IF YOU DON'T DO THIS

mod db;
mod api;

use dotenv::dotenv;
use env_logger::Env;
use log::info;

use crate::db::dbconn;
use crate::api::buildings_api::*;
use crate::api::rooms_api::*;
use crate::api::storeys_api::*;

use actix_web::{middleware::Logger, middleware::NormalizePath, web, middleware::DefaultHeaders, App, HttpServer};

use std::io::{ErrorKind, Error};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    info!("initializing logging...");
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
   

    // r2d2 will attempt to connect until postgres is up, don't let the error messages irritate you 
    info!("attempting to connect to database service...");
    if dbconn::init().is_err() { return Err(Error::new(ErrorKind::Other, "could not connect to DB service")); }
    info!("database connection successful");

    // ...and here we go!
    info!("starting API service");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(DefaultHeaders::new().add(("Content-Type", "application/json")))
            .wrap(NormalizePath::trim())
            .service(
                web::scope("/assets")
                    .service(get_all_buildings)
                    .service(get_building_by_id)
                    .service(get_all_storeys)
                    .service(get_storeys_by_building)
                    .service(get_storey_by_id)
                    .service(get_all_rooms)
                    .service(get_rooms_by_storey)
                    .service(get_room_by_id)
                    .service(add_building)
                    .service(update_building)
                    .service(delete_building)
                    .service(add_storey)
                    .service(update_storey)
                    .service(delete_storey)
                    .service(add_room)
                    .service(update_room)
                    .service(delete_room)
            )
    }).bind(("0.0.0.0", 9000))?.run().await // HAS to be 0.0.0.0 or docker won't let you connect
}
