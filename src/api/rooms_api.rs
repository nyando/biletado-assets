use actix_web::{get, post, put, delete, HttpResponse, Responder, web};
use serde_json::{json};

use crate::db::crud::rooms_crud::*;
use crate::db::crud::storeys_crud::find_storey_by_id;
use crate::db::models::OptionalIDRoom;

use uuid::Uuid;

#[get("/assets/rooms")]
async fn get_all_rooms() -> impl Responder {
    let result = serde_json::to_string(&get_rooms()).unwrap();
    info!("found {} rooms", result.len());
    HttpResponse::Ok().json(result)
}

#[post("/assets/rooms")]
async fn add_room(req_body: String) -> impl Responder {

    let body_content : Result<OptionalIDRoom, serde_json::Error> = serde_json::from_str(&req_body);
    if let Err(_) = body_content { 
        error!("invalid room request body: {}", req_body);
        return HttpResponse::BadRequest().json(json!({ "message": "invalid input" }));
    }
    
    let room = body_content.unwrap();
    let room_name = room.name.to_string();
    let room_storey_id = room.storey_id;

    if let None = find_storey_by_id(room_storey_id) {
        error!("storey with UUID {} does not exist", room_storey_id);
        return HttpResponse::UnprocessableEntity().json(json!({ "message": "invalid storey UUID" }));
    }
    
    if let Some(new_room) = create_or_update_room(room.id, room_name, room_storey_id) {
        info!("room {} newly created or updated", new_room.id);
        HttpResponse::Created().json(new_room)
    } else {
        error!("room create/update threw an error");
        HttpResponse::InternalServerError().json(json!({ "message": "something went wrong :O" }))
    }
}

#[get("/assets/rooms/{id}")]
async fn get_room_by_id(id: web::Path<String>) -> impl Responder {

    let room_uuid = Uuid::parse_str(&id);

    if let Ok(room_id) = room_uuid {

        let result = serde_json::to_string(&find_room_by_id(room_id));

        if result.is_ok() { 
            info!("found room with UUID: {}", id);
            HttpResponse::Ok().json(result.unwrap())
        } else {
            error!("could not find room with UUID: {}", id);
            HttpResponse::NotFound().json(json!({ "message": "room with UUID not found" }))
        }

    } else {
        error!("failed to parse room UUID: {}", id);
        HttpResponse::NotFound().json(json!({ "message": "invalid UUID" }))
    }

}

#[put("/assets/rooms/{id}")]
async fn update_room(id: web::Path<String>, req_body: String) -> impl Responder {

    let param_id = Uuid::parse_str(&id);
    if let Err(_) = param_id {
        error!("invalid param UUID: {}", id);
        return HttpResponse::BadRequest().json(json!({ "message": "invalid UUID in parameters" }));
    }
    
    let body_content : Result<OptionalIDRoom, serde_json::Error> = serde_json::from_str(&req_body);
    if let Err(_) = body_content {
        error!("invalid room request body: {}", req_body);
        return HttpResponse::BadRequest().json(json!({ "message": "invalid input" }));
    }

    let room = body_content.unwrap();
    let room_name = room.name.to_string();
    let room_storey_id = room.storey_id;

    if let Some(body_id) = room.id {
        let param_id = param_id.unwrap();
        if param_id != body_id {
            error!("request parameter UUID {} and body UUID {} do not match", param_id, body_id);
            return HttpResponse::UnprocessableEntity().json(json!({ "message": "mismatched ID in URL and object" }));
        }
    }

    if let None = find_storey_by_id(room_storey_id) {
        error!("storey with UUID {} does not exist", room_storey_id);
        return HttpResponse::UnprocessableEntity().json(json!({ "message": "invalid storey UUID" }));
    }
    
    if let Some(new_room) = create_or_update_room(room.id, room_name, room_storey_id) {
        info!("room {} newly created or updated", new_room.id);
        HttpResponse::NoContent().finish()
    } else {
        error!("room create/update threw an error");
        HttpResponse::InternalServerError().json(json!({ "message": "something went wrong :O" }))
    }

}

#[delete("/assets/rooms/{id}")]
async fn delete_room(id: web::Path<String>) -> impl Responder {

    let param_id = Uuid::parse_str(&id);
    if let Err(_) = param_id {
        error!("invalid param UUID: {}", id);
        return HttpResponse::BadRequest().json(json!({ "message": "invalid UUID in parameters" }));
    }

    // TODO check that no reservations exist before deleting room
    
    let param_id = param_id.unwrap();
    if delete_room_by_id(param_id) {
        info!("deleted room {}", param_id);
        HttpResponse::NoContent().finish()
    } else {
        error!("room with UUID {} not found", param_id);
        HttpResponse::NotFound().finish()
    }

}