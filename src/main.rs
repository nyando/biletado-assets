#[macro_use]
extern crate diesel;
mod db;

use actix_web::{get, App, HttpServer, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello)
    }).bind(("127.0.0.1", 8081))?.run().await
}