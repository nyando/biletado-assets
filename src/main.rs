#[macro_use]
extern crate diesel;
mod db;

mod api;
use crate::api::assets_api::get_all_buildings;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(get_all_buildings)
    }).bind(("127.0.0.1", 8081))?.run().await
}