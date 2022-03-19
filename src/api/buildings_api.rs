use actix_web::{get, post, put, delete, HttpResponse, Responder, web};
use serde_json::{json};

use crate::db::crud::buildings_crud::*;
use crate::db::crud::storeys_crud::has_storeys;
use crate::db::models::OptionalIDBuilding;

use uuid::Uuid;

#[get("/assets/buildings")]
async fn get_all_buildings() -> impl Responder {
    let result = serde_json::to_string(&get_buildings()).unwrap();
    info!("found {} buildings", result.len());
    HttpResponse::Ok().json(result)
}

#[post("/assets/buildings")]
async fn add_building(req_body: String) -> impl Responder {
    let body_content : Result<OptionalIDBuilding, serde_json::Error> = serde_json::from_str(&req_body);
    if let Err(_) = body_content {
        error!("invalid building request body: {}", req_body);
        return HttpResponse::BadRequest().json(json!({ "message": "invalid input" }));
    }
    
    let building = body_content.unwrap();
    let building_name = building.name.to_string();
    let building_address = building.address.to_string();

    if let Some(new_building) = create_or_update_building(building.id, building_name, building_address) {
        info!("building {} newly created or updated", new_building.id);
        HttpResponse::Created().json(new_building)
    } else {
        error!("building create/update threw an error");
        HttpResponse::InternalServerError().json(json!({ "message": "something went wrong :O" }))
    }
}

#[get("/assets/buildings/{id}")]
async fn get_building_by_id(id: web::Path<String>) -> impl Responder {
    let building_uuid = Uuid::parse_str(&id);

    if let Ok(building_id) = building_uuid {

        let result = serde_json::to_string(&find_building_by_id(building_id));

        if result.is_ok() { 
            info!("found building with UUID: {}", id);
            HttpResponse::Ok().json(result.unwrap())
        } else {
            error!("could not find building with UUID: {}", id);
            HttpResponse::NotFound().json(json!({ "message": "building with UUID not found" }))
        }

    } else {
        error!("failed to parse building UUID: {}", id);
        HttpResponse::NotFound().json(json!({ "message": "invalid UUID" }))
    }
}

#[put("/assets/buildings/{id}")]
async fn update_building(id: web::Path<String>, req_body: String) -> impl Responder {
    
    let param_id = Uuid::parse_str(&id);
    if let Err(_) = param_id {
        error!("invalid param UUID: {}", id);
        return HttpResponse::BadRequest().json(json!({ "message": "invalid UUID in parameters" }));
    }
    
    let body_content : Result<OptionalIDBuilding, serde_json::Error> = serde_json::from_str(&req_body);
    if let Err(_) = body_content {
        error!("invalid building request body: {}", req_body);
        return HttpResponse::BadRequest().json(json!({ "message": "invalid input" }));
    }

    let building = body_content.unwrap();
    let building_name = building.name.to_string();
    let building_address = building.address.to_string();

    if let Some(body_id) = building.id {
        let param_id = param_id.unwrap();
        if param_id != body_id {
            error!("request parameter UUID {} and body UUID {} do not match", param_id, body_id);
            return HttpResponse::UnprocessableEntity().json(json!({ "message": "mismatched ID in URL and object" }));
        }
    }

    if let Some(new_building) = create_or_update_building(building.id, building_name, building_address) {
        info!("building {} newly created or updated", new_building.id);
        HttpResponse::NoContent().finish()
    } else {
        error!("building create/update threw an error");
        HttpResponse::InternalServerError().json(json!({ "message": "something went wrong :O" }))
    }

}

#[delete("/assets/buildings/{id}")]
async fn delete_building(id: web::Path<String>) -> impl Responder {

    let param_id = Uuid::parse_str(&id);
    if let Err(_) = param_id {
        error!("invalid param UUID: {}", id);
        return HttpResponse::BadRequest().json(json!({ "message": "invalid UUID in parameters" }));
    }

    let param_id = param_id.unwrap();
    if has_storeys(param_id) {
        error!("cannot delete building {}, has existing storeys", param_id);
        HttpResponse::UnprocessableEntity().json(json!({ "message": "building has existing storeys" }));
    }
    
    if delete_building_by_id(param_id) {
        info!("deleted building {}", param_id);
        HttpResponse::NoContent().finish()
    } else {
        error!("building with UUID {} not found", param_id);
        HttpResponse::NotFound().finish()
    }
    
}
