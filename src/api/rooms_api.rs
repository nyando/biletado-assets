use actix_web::{get, post, put, delete, HttpResponse, Responder, web};
use serde_json::{json};

use crate::db::rooms_crud::*;
use crate::db::storeys_crud::find_storey_by_id;
use crate::db::models::OptionalIDRoom;

use uuid::Uuid;

#[get("/assets/rooms")]
async fn get_all_rooms() -> impl Responder {
    let result = serde_json::to_string(&get_rooms()).unwrap();
    HttpResponse::Ok().json(result)
}

#[post("/assets/rooms")]
async fn add_room(req_body: String) -> impl Responder {

    let body_content : Result<OptionalIDRoom, serde_json::Error> = serde_json::from_str(&req_body);
    if let Err(_) = body_content { return HttpResponse::BadRequest().json(json!({ "message": "invalid input" })); }
    
    let room = body_content.unwrap();
    let room_name = room.name.to_string();
    let room_storey_id = room.storey_id;

    if let None = find_storey_by_id(room_storey_id) {
        return HttpResponse::UnprocessableEntity().json(json!({ "message": "invalid storey UUID" }));
    }
    
    if let Some(new_room) = create_or_update_room(room.id, room_name, room_storey_id) {
        HttpResponse::Created().json(new_room)
    } else {
        HttpResponse::InternalServerError().json(json!({ "message": "something went wrong :O" }))
    }
}

#[get("/assets/rooms/{id}")]
async fn get_room_by_id(id: web::Path<String>) -> impl Responder {

    let room_uuid = Uuid::parse_str(&id);

    if let Ok(room_id) = room_uuid {

        let result = serde_json::to_string(&find_room_by_id(room_id));

        if result.is_ok() { 
            HttpResponse::Ok().json(result.unwrap())
        } else {
            HttpResponse::NotFound().json(json!({ "message": "room with UUID not found" }))
        }

    } else {
        HttpResponse::NotFound().json(json!({ "message": "invalid UUID" }))
    }

}

#[put("/assets/rooms/{id}")]
async fn update_room(id: web::Path<String>, req_body: String) -> impl Responder {

    let param_id = Uuid::parse_str(&id);
    if let Err(_) = param_id { return HttpResponse::BadRequest().json(json!({ "message": "invalid UUID in parameters" })); }
    
    let body_content : Result<OptionalIDRoom, serde_json::Error> = serde_json::from_str(&req_body);
    if let Err(_) = body_content { return HttpResponse::BadRequest().json(json!({ "message": "invalid input" })); }

    let room = body_content.unwrap();
    let room_name = room.name.to_string();
    let room_storey_id = room.storey_id;

    if let Some(body_id) = room.id {
        if param_id.unwrap() != body_id {
            return HttpResponse::UnprocessableEntity().json(json!({ "message": "mismatched ID in URL and object" }));
        }
    }

    if let None = find_storey_by_id(room_storey_id) {
        return HttpResponse::UnprocessableEntity().json(json!({ "message": "invalid storey UUID" }));
    }
    
    if let Some(_) = create_or_update_room(room.id, room_name, room_storey_id) {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::InternalServerError().json(json!({ "message": "something went wrong :O" }))
    }

}

#[delete("/assets/rooms/{id}")]
async fn delete_room(id: web::Path<String>) -> impl Responder {

    let param_id = Uuid::parse_str(&id);
    if let Err(_) = param_id { return HttpResponse::BadRequest().json(json!({ "message": "invalid UUID in parameters" })); }

    // TODO check that no reservations exist before deleting room
    
    if delete_room_by_id(param_id.unwrap()) {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }

}