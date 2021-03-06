use actix_web::{dev::ServiceRequest, get, post, put, delete, HttpRequest, HttpResponse, Responder, web};
use actix_web_httpauth::middleware::HttpAuthentication;

use log::{debug, info, error};
use serde_json::json;
use serde::Deserialize;
use std::env;

use crate::api::auth::validator;
use crate::api::util::{get_jaeger_params, validate_uuid};
use crate::db::crud::rooms_crud::*;
use crate::db::crud::storeys_crud::find_storey_by_id;
use crate::db::models::OptionalIDRoom;
use crate::db::models::Reservation;

#[derive(Debug, Deserialize)]
pub struct QueryByStorey {
    storey_id: Option<uuid::Uuid>
}

#[get("/rooms")]
async fn get_rooms_by_storey(param: web::Query<QueryByStorey>) -> impl Responder {
    let rooms = if param.storey_id.is_some() {
        rooms_by_storey(param.storey_id.unwrap())
    } else {
        get_rooms()
    };
    info!("found {} rooms", rooms.len());
    HttpResponse::Ok().json(rooms)
}

#[post("/rooms", wrap="HttpAuthentication::bearer(validator)")]
async fn add_room(req_body: String) -> impl Responder {

    let body_content : Result<OptionalIDRoom, serde_json::Error> = serde_json::from_str(&req_body);
    if body_content.is_err() { 
        error!("invalid room request body: {}", req_body);
        return HttpResponse::BadRequest().json(json!({ "message": "invalid input" }));
    }
    
    let room = body_content.unwrap();
    let room_name = room.name.to_string();
    let room_storey_id = room.storey_id;

    if find_storey_by_id(room_storey_id).is_none() {
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

#[get("/rooms/{id}")]
async fn get_room_by_id(id: web::Path<String>) -> impl Responder {

    let room_uuid = validate_uuid(id.to_string());

    if let Some(room_id) = room_uuid {

        match find_room_by_id(room_id) {
            Some(room) => {
                info!("found room with UUID: {}", id);
                HttpResponse::Ok().json(room)
            },
            None => {
                error!("could not find room with UUID: {}", id);
                HttpResponse::NotFound().json(json!({ "message": "room with UUID not found" }))
            }
        }

    } else {
        error!("failed to parse room UUID: {}", id);
        HttpResponse::NotFound().json(json!({ "message": "invalid UUID" }))
    }

}

#[put("/rooms/{id}", wrap="HttpAuthentication::bearer(validator)")]
async fn update_room(id: web::Path<String>, req_body: String) -> impl Responder {

    let param_id = validate_uuid(id.to_string());
    if param_id.is_none() {
        error!("invalid param UUID: {}", id);
        return HttpResponse::BadRequest().json(json!({ "message": "invalid UUID in parameters" }));
    }
    
    let body_content : Result<OptionalIDRoom, serde_json::Error> = serde_json::from_str(&req_body);
    if body_content.is_err() {
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

    if find_storey_by_id(room_storey_id).is_none() {
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

#[delete("/rooms/{id}", wrap="HttpAuthentication::bearer(validator)")]
async fn delete_room(id: web::Path<String>, req: HttpRequest) -> impl Responder {
    
    let param_id = validate_uuid(id.to_string());
    if param_id.is_none() {
        error!("invalid param UUID: {}", id);
        return HttpResponse::BadRequest().json(json!({ "message": "invalid UUID in parameters" }));
    }

    let param_id = param_id.unwrap();
    let (jaeger_key, jaeger_id) = get_jaeger_params(&ServiceRequest::from_request(req));
    if let Some(has_reservations) = has_room_reservations(param_id, &jaeger_key, &jaeger_id) {
        if has_reservations {
            info!("room {} has existing reservations, cannot delete", param_id);
            return HttpResponse::UnprocessableEntity().json(
                json!({ "message": format!("room {} has existing reservations", param_id) })
            );
        } else {
            info!("room {} has no associated reservations, ok to delete", param_id);
        }
    } else {
        error!("error getting reservation data for room {}, not deleting", param_id);
        return HttpResponse::NotFound().finish();
    }
    
    if delete_room_by_id(param_id) {
        info!("deleted room {}", param_id);
        HttpResponse::NoContent().finish()
    } else {
        error!("room with UUID {} not found", param_id);
        HttpResponse::NotFound().finish()
    }

}

fn has_room_reservations(delete_room_id: uuid::Uuid, jaeger_key: &str, jaeger_id: &str) -> Option<bool> {

    let reservations_host = env::var("RESERVATIONS_HOST").expect("RESERVATIONS_HOST variable not set");
    let reservations_port = env::var("RESERVATIONS_PORT").expect("RESERVATIONS_PORT variable not set");
    
    let reservations_url  = format!("http://{}:{}/api/reservations/", reservations_host, reservations_port);

    let client = reqwest::blocking::Client::new();
    let resp = client.get(reservations_url)
                     .header(jaeger_key, jaeger_id)
                     .send().ok()?;
    
    if resp.status().is_success() {
        let reservations : Vec<Reservation> = resp.json().ok()?;
        debug!("received reservations from backend");
        Some(reservations.iter().any(|res| res.room_id == delete_room_id))
    } else {
        debug!("no reservations found");
        None
    }
}