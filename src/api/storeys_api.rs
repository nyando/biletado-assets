use actix_web::{get, post, put, delete, HttpResponse, Responder, web};
use actix_web_httpauth::middleware::HttpAuthentication;

use log::{info, error};
use serde_json::json;
use serde::Deserialize;

use crate::api::auth::validator;
use crate::api::util::validate_uuid;
use crate::db::crud::storeys_crud::*;
use crate::db::crud::rooms_crud::has_rooms;
use crate::db::crud::buildings_crud::find_building_by_id;
use crate::db::models::OptionalIDStorey;

#[derive(Debug, Deserialize)]
pub struct QueryByBuilding {
    building_id: Option<uuid::Uuid>
}

#[get("/storeys")]
async fn get_storeys_by_building(param: web::Query<QueryByBuilding>) -> impl Responder {
    let storeys = if param.building_id.is_some() {
        storeys_by_building(param.building_id.unwrap())
    } else {
        get_storeys()
    };
    info!("found {} storeys", storeys.len());
    HttpResponse::Ok().json(storeys)
}

#[post("/storeys", wrap="HttpAuthentication::bearer(validator)")]
async fn add_storey(req_body: String) -> impl Responder {

    let body_content : Result<OptionalIDStorey, serde_json::Error> = serde_json::from_str(&req_body);
    if body_content.is_err() {
        error!("invalid storey request body: {}", req_body);
        return HttpResponse::BadRequest().json(json!({ "message": "invalid input" }));
    }
    
    let storey = body_content.unwrap();
    let storey_name = storey.name.to_string();
    let storey_building_id = storey.building_id;

    if find_building_by_id(storey_building_id).is_none() {
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

#[get("/storeys/{id}")]
async fn get_storey_by_id(id: web::Path<String>) -> impl Responder {
    
    let storey_uuid = validate_uuid(id.to_string());

    if let Some(storey_id) = storey_uuid {

        match find_storey_by_id(storey_id) {
            Some(storey) => {
                info!("found storey with UUID: {}", id);
                HttpResponse::Ok().json(storey)
            },
            None => {
                error!("could not find storey with UUID: {}", id);
                HttpResponse::NotFound().json(json!({ "message": "storey with UUID not found" }))
            }
        }

    } else {
        error!("failed to parse storey UUID: {}", id);
        HttpResponse::NotFound().json(json!({ "message": "invalid UUID" }))
    }
}

#[put("/storeys/{id}", wrap="HttpAuthentication::bearer(validator)")]
async fn update_storey(id: web::Path<String>, req_body: String) -> impl Responder {

    let param_id = validate_uuid(id.to_string());
    if param_id.is_none() {
        error!("invalid param UUID: {}", id);
        return HttpResponse::BadRequest().json(json!({ "message": "invalid UUID in parameters" }));
    }
    
    let body_content : Result<OptionalIDStorey, serde_json::Error> = serde_json::from_str(&req_body);
    if body_content.is_err() {
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

    if find_building_by_id(storey_building_id).is_none() {
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

#[delete("/storeys/{id}", wrap="HttpAuthentication::bearer(validator)")]
async fn delete_storey(id: web::Path<String>) -> impl Responder {

    let param_id = validate_uuid(id.to_string());
    if param_id.is_none() {
        error!("invalid param UUID: {}", id);
        return HttpResponse::BadRequest().json(json!({ "message": "invalid UUID in parameters" }));
    }

    let param_id = param_id.unwrap();
    if has_rooms(param_id) {
        error!("cannot delete storey {}, has existing rooms", param_id);
        return HttpResponse::UnprocessableEntity().json(json!({ "message": "storey has existing rooms" }));
    }
    
    if delete_storey_by_id(param_id) {
        info!("deleted storey {}", param_id);
        HttpResponse::NoContent().finish()
    } else {
        error!("storey with UUID {} not found", param_id);
        HttpResponse::NotFound().finish()
    }

}
