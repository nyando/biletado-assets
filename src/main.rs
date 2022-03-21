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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    dbconn::init();

    info!("assets API service running on port 8081");
    
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
    }).bind(("127.0.0.1", 8081))?.run().await
}