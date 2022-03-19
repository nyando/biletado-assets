use actix_web::{get, post, put, delete, HttpResponse, Responder, web};
use serde_json::{json};

use crate::db::crud::buildings_crud::*;
use crate::db::crud::storeys_crud::has_storeys;
use crate::db::models::OptionalIDBuilding;

use uuid::Uuid;

#[get("/assets/buildings")]
async fn get_all_buildings() -> impl Responder {
    let result = serde_json::to_string(&get_buildings()).unwrap();
    HttpResponse::Ok().json(result)
}

#[post("/assets/buildings")]
async fn add_building(req_body: String) -> impl Responder {
    let body_content : Result<OptionalIDBuilding, serde_json::Error> = serde_json::from_str(&req_body);
    if let Err(_) = body_content { return HttpResponse::BadRequest().json(json!({ "message": "invalid input" })); }
    
    let building = body_content.unwrap();
    let building_name = building.name.to_string();
    let building_address = building.address.to_string();

    if let Some(new_building) = create_or_update_building(building.id, building_name, building_address) {
        HttpResponse::Created().json(new_building)
    } else {
        HttpResponse::InternalServerError().json(json!({ "message": "something went wrong :O" }))
    }
}

#[get("/assets/buildings/{id}")]
async fn get_building_by_id(id: web::Path<String>) -> impl Responder {
    let building_uuid = Uuid::parse_str(&id);

    if let Ok(building_id) = building_uuid {

        let result = serde_json::to_string(&find_building_by_id(building_id));

        if result.is_ok() { 
            HttpResponse::Ok().json(result.unwrap())
        } else {
            HttpResponse::NotFound().json(json!({ "message": "building with UUID not found" }))
        }

    } else {
        HttpResponse::NotFound().json(json!({ "message": "invalid UUID" }))
    }
}

#[put("/assets/buildings/{id}")]
async fn update_building(id: web::Path<String>, req_body: String) -> impl Responder {
    
    let param_id = Uuid::parse_str(&id);
    if let Err(_) = param_id { return HttpResponse::BadRequest().json(json!({ "message": "invalid UUID in parameters" })); }
    
    let body_content : Result<OptionalIDBuilding, serde_json::Error> = serde_json::from_str(&req_body);
    if let Err(_) = body_content { return HttpResponse::BadRequest().json(json!({ "message": "invalid input" })); }

    let building = body_content.unwrap();
    let building_name = building.name.to_string();
    let building_address = building.address.to_string();

    if let Some(body_id) = building.id {
        if param_id.unwrap() != body_id {
            return HttpResponse::UnprocessableEntity().json(json!({ "message": "mismatched ID in URL and object" }));
        }
    }

    if let Some(_) = create_or_update_building(building.id, building_name, building_address) {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::InternalServerError().json(json!({ "message": "something went wrong :O" }))
    }

}

#[delete("/assets/buildings/{id}")]
async fn delete_building(id: web::Path<String>) -> impl Responder {

    let param_id = Uuid::parse_str(&id);
    if let Err(_) = param_id { return HttpResponse::BadRequest().json(json!({ "message": "invalid UUID in parameters" })); }

    if has_storeys(*param_id.as_ref().unwrap()) {
        HttpResponse::UnprocessableEntity().json(json!({ "message": "building has existing storeys" }));
    }
    
    if delete_building_by_id(param_id.unwrap()) {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
    
}
