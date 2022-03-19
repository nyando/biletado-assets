use actix_web::{get, post, put, delete, HttpResponse, Responder, web};
use serde_json::{json};

use crate::db::crud::storeys_crud::*;
use crate::db::crud::rooms_crud::has_rooms;
use crate::db::crud::buildings_crud::find_building_by_id;
use crate::db::models::OptionalIDStorey;

use uuid::Uuid;

#[get("/assets/storeys")]
async fn get_all_storeys() -> impl Responder {
    let result = serde_json::to_string(&get_storeys()).unwrap();
    info!("found {} storeys", result.len());
    HttpResponse::Ok().json(result)
}

#[post("/assets/storeys")]
async fn add_storey(req_body: String) -> impl Responder {

    let body_content : Result<OptionalIDStorey, serde_json::Error> = serde_json::from_str(&req_body);
    if let Err(_) = body_content {
        error!("invalid storey request body: {}", req_body);
        return HttpResponse::BadRequest().json(json!({ "message": "invalid input" }));
    }
    
    let storey = body_content.unwrap();
    let storey_name = storey.name.to_string();
    let storey_building_id = storey.building_id;

    if let None = find_building_by_id(storey_building_id) {
        error!("building with UUID {} does not exist", storey_building_id);
        return HttpResponse::UnprocessableEntity().json(json!({ "message": "invalid building UUID" }));
    }
    
    if let Some(new_storey) = create_or_update_storey(storey.id, storey_name, storey_building_id) {
        info!("storey {} newly created or updated", new_storey.id);
        HttpResponse::Created().json(new_storey)
    } else {
        error!("storey create/update threw an error");
        HttpResponse::InternalServerError().json(json!({ "message": "something went wrong :O" }))
    }

}

#[get("/assets/storeys/{id}")]
async fn get_storey_by_id(id: web::Path<String>) -> impl Responder {
    let storey_uuid = Uuid::parse_str(&id);

    if let Ok(storey_id) = storey_uuid {

        let result = serde_json::to_string(&find_storey_by_id(storey_id));

        if result.is_ok() { 
            info!("found storey with UUID: {}", id);
            HttpResponse::Ok().json(result.unwrap())
        } else {
            error!("could not find storey with UUID: {}", id);
            HttpResponse::NotFound().json(json!({ "message": "storey with UUID not found" }))
        }

    } else {
        error!("failed to parse storey UUID: {}", id);
        HttpResponse::NotFound().json(json!({ "message": "invalid UUID" }))
    }
}

#[put("/assets/storeys/{id}")]
async fn update_storey(id: web::Path<String>, req_body: String) -> impl Responder {

    let param_id = Uuid::parse_str(&id);
    if let Err(_) = param_id {
        error!("invalid param UUID: {}", id);
        return HttpResponse::BadRequest().json(json!({ "message": "invalid UUID in parameters" }));
    }
    
    let body_content : Result<OptionalIDStorey, serde_json::Error> = serde_json::from_str(&req_body);
    if let Err(_) = body_content {
        error!("invalid storey request body: {}", req_body);
        return HttpResponse::BadRequest().json(json!({ "message": "invalid input" }));
    }

    let storey = body_content.unwrap();
    let storey_name = storey.name.to_string();
    let storey_building_id = storey.building_id;

    if let Some(body_id) = storey.id {
        let param_id = param_id.unwrap();
        if param_id != body_id {
            error!("request parameter UUID {} and body UUID {} do not match", param_id, body_id);
            return HttpResponse::UnprocessableEntity().json(json!({ "message": "mismatched ID in URL and object" }));
        }
    }

    if let None = find_building_by_id(storey_building_id) {
        error!("building with UUID {} does not exist", storey_building_id);
        return HttpResponse::UnprocessableEntity().json(json!({ "message": "invalid building UUID" }));
    }
    
    if let Some(new_storey) = create_or_update_storey(storey.id, storey_name, storey_building_id) {
        info!("storey {} newly created or updated", new_storey.id);
        HttpResponse::NoContent().finish()
    } else {
        error!("storey create/update threw an error");
        HttpResponse::InternalServerError().json(json!({ "message": "something went wrong :O" }))
    }

}

#[delete("/assets/storeys/{id}")]
async fn delete_storey(id: web::Path<String>) -> impl Responder {

    let param_id = Uuid::parse_str(&id);
    if let Err(_) = param_id {
        error!("invalid param UUID: {}", id);
        return HttpResponse::BadRequest().json(json!({ "message": "invalid UUID in parameters" }));
    }

    let param_id = param_id.unwrap();
    if has_rooms(param_id) {
        error!("cannot delete storey {}, has existing rooms", param_id);
        HttpResponse::UnprocessableEntity().json(json!({ "message": "storey has existing rooms" }));
    }
    
    if delete_storey_by_id(param_id) {
        info!("deleted storey {}", param_id);
        HttpResponse::NoContent().finish()
    } else {
        error!("storey with UUID {} not found", param_id);
        HttpResponse::NotFound().finish()
    }

}
