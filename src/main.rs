mod db;

#[macro_use]
extern crate diesel;

mod api;

use crate::api::assets_api::*;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_all_buildings)
            .service(add_building)
            .service(get_building_by_id)
    }).bind(("127.0.0.1", 8081))?.run().await
}