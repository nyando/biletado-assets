use actix_web::{get, post, put, delete, HttpResponse, Responder, web};
use serde_json::{json};

use crate::db::crudops::*;
use crate::db::models::OptionalIDBuilding;
use crate::db::models::OptionalIDStorey;
use crate::db::models::OptionalIDRoom;

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

#[get("/assets/storeys")]
async fn get_all_storeys() -> impl Responder {
    let result = serde_json::to_string(&get_storeys()).unwrap();
    HttpResponse::Ok().json(result)
}

#[post("/assets/storeys")]
async fn add_storey(req_body: String) -> impl Responder {

    let body_content : Result<OptionalIDStorey, serde_json::Error> = serde_json::from_str(&req_body);
    if let Err(_) = body_content { return HttpResponse::BadRequest().json(json!({ "message": "invalid input" })); }
    
    let storey = body_content.unwrap();
    let storey_name = storey.name.to_string();
    let storey_building_id = storey.building_id;

    if let None = find_building_by_id(storey_building_id) {
        return HttpResponse::UnprocessableEntity().json(json!({ "message": "invalid building UUID" }));
    }
    
    if let Some(new_storey) = create_or_update_storey(storey.id, storey_name, storey_building_id) {
        HttpResponse::Created().json(new_storey)
    } else {
        HttpResponse::InternalServerError().json(json!({ "message": "something went wrong :O" }))
    }

}

#[get("/assets/storeys/{id}")]
async fn get_storey_by_id(id: web::Path<String>) -> impl Responder {
    let storey_uuid = Uuid::parse_str(&id);

    if let Ok(storey_id) = storey_uuid {

        let result = serde_json::to_string(&find_storey_by_id(storey_id));

        if result.is_ok() { 
            HttpResponse::Ok().json(result.unwrap())
        } else {
            HttpResponse::NotFound().json(json!({ "message": "storey with UUID not found" }))
        }

    } else {
        HttpResponse::NotFound().json(json!({ "message": "invalid UUID" }))
    }
}

#[put("/assets/storeys/{id}")]
async fn update_storey(id: web::Path<String>, req_body: String) -> impl Responder {

    let param_id = Uuid::parse_str(&id);
    if let Err(_) = param_id { return HttpResponse::BadRequest().json(json!({ "message": "invalid UUID in parameters" })); }
    
    let body_content : Result<OptionalIDStorey, serde_json::Error> = serde_json::from_str(&req_body);
    if let Err(_) = body_content { return HttpResponse::BadRequest().json(json!({ "message": "invalid input" })); }

    let storey = body_content.unwrap();
    let storey_name = storey.name.to_string();
    let storey_building_id = storey.building_id;

    if let Some(body_id) = storey.id {
        if param_id.unwrap() != body_id {
            return HttpResponse::UnprocessableEntity().json(json!({ "message": "mismatched ID in URL and object" }));
        }
    }

    if let None = find_building_by_id(storey_building_id) {
        return HttpResponse::UnprocessableEntity().json(json!({ "message": "invalid building UUID" }));
    }
    
    if let Some(_) = create_or_update_storey(storey.id, storey_name, storey_building_id) {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::InternalServerError().json(json!({ "message": "something went wrong :O" }))
    }

}

#[delete("/assets/storeys/{id}")]
async fn delete_storey(id: web::Path<String>) -> impl Responder {

    let param_id = Uuid::parse_str(&id);
    if let Err(_) = param_id { return HttpResponse::BadRequest().json(json!({ "message": "invalid UUID in parameters" })); }

    if has_rooms(*param_id.as_ref().unwrap()) {
        HttpResponse::UnprocessableEntity().json(json!({ "message": "storey has existing rooms" }));
    }
    
    if delete_storey_by_id(param_id.unwrap()) {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }

}

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
    
    if let Some(_) = create_or_update_storey(room.id, room_name, room_storey_id) {
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