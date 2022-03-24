mod db;

extern crate openssl;

#[macro_use]
extern crate diesel;

mod api;

use dotenv::dotenv;
use env_logger::Env;
use log::info;

use crate::db::dbconn;
use crate::api::buildings_api::*;
use crate::api::rooms_api::*;
use crate::api::storeys_api::*;
use crate::api::auth::validator;

use actix_web::{middleware::Logger, middleware::NormalizePath, web, middleware::DefaultHeaders, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;

use std::io::{ErrorKind, Error};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    info!("initializing logging...");
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    info!("attempting to connect to database service...");
    if dbconn::init().is_err() { return Err(Error::new(ErrorKind::Other, "could not connect to DB service")); }
    info!("database connection successful");

    info!("starting API service");
    HttpServer::new(|| {

        let jwtauth = HttpAuthentication::bearer(validator);

        App::new()
            .wrap(Logger::default())
            .wrap(DefaultHeaders::new().add(("Content-Type", "application/json")))
            .wrap(NormalizePath::trim())
            .service(
                web::scope("/assets")
                    .service(get_all_buildings)
                    .service(get_building_by_id)
                    .service(get_all_storeys)
                    .service(get_storey_by_id)
                    .service(get_all_rooms)
                    .service(get_room_by_id)
            )
            .wrap(jwtauth)
            .service(
                web::scope("/assets")
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
    }).bind(("0.0.0.0", 9000))?.run().await
}
