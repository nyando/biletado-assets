use actix_web::{get, post, put, delete, HttpResponse, Responder, web};
use serde_json::json;

use crate::db::crudops::*;

use uuid::Uuid;

#[get("/assets/buildings")]
async fn get_all_buildings() -> impl Responder {
    let result = serde_json::to_string(&get_buildings()).unwrap();
    HttpResponse::Ok().json(result)
}

#[post("/assets/buildings")]
async fn add_building(req_body: String) -> impl Responder {
    HttpResponse::Created().json(req_body)
}

#[get("/assets/buildings/{id}")]
async fn get_building_by_id(id: web::Path<String>) -> impl Responder {
    let building_uuid = Uuid::parse_str(&id);
    match building_uuid {
        Ok(building_id) => {
            let result = serde_json::to_string(&find_building_by_id(building_id));
            if result.is_ok() { 
                HttpResponse::Ok().json(result.unwrap())
            } else { HttpResponse::NotFound().json(json!({ "message": "building with UUID not found" })) }
        },
        _ => HttpResponse::NotFound().json(json!({ "message": "invalid UUID" }))
    }
}

#[put("/assets/buildings/{id}")]
async fn update_building(_id: web::Path<String>, _req_body: String) -> impl Responder {
    HttpResponse::NoContent()
}

#[delete("/assets/buildings/{id}")]
async fn delete_building(_id: web::Path<String>) -> impl Responder {
    HttpResponse::NoContent()
}

#[get("/assets/storeys")]
async fn get_all_storeys() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/assets/storeys")]
async fn add_storey(req_body: String) -> impl Responder {
    HttpResponse::Created().body(req_body)
}

#[get("/assets/storeys/{id}")]
async fn get_storey_by_id(_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok()
}

#[put("/assets/storeys/{id}")]
async fn update_storey(_id: web::Path<String>, _req_body: String) -> impl Responder {
    HttpResponse::NoContent()
}

#[delete("/assets/storeys/{id}")]
async fn delete_storey(_id: web::Path<String>) -> impl Responder {
    HttpResponse::NoContent()
}

#[get("/assets/rooms")]
async fn get_all_rooms() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/assets/rooms")]
async fn add_room(req_body: String) -> impl Responder {
    HttpResponse::Created().body(req_body)
}

#[get("/assets/rooms/{id}")]
async fn get_room_by_id(_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok()
}

#[put("/assets/rooms/{id}")]
async fn update_room(_id: web::Path<String>, _req_body: String) -> impl Responder {
    HttpResponse::NoContent()
}

#[delete("/assets/rooms/{id}")]
async fn delete_room(_id: web::Path<String>) -> impl Responder {
    HttpResponse::NoContent()
}