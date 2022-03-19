use uuid::Uuid;

use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::select;
use diesel::dsl::exists;

use crate::db::models::*;
use crate::db::schema::buildings::dsl::buildings;
use crate::db::schema::buildings::name as b_name;
use crate::db::schema::buildings::address as b_address;

use crate::db::schema::storeys::dsl::storeys;
use crate::db::schema::storeys::building_id;
use crate::db::schema::storeys::name as s_name;

use crate::db::schema::rooms::dsl::rooms;
use crate::db::schema::rooms::storey_id;
use crate::db::schema::rooms::name as r_name;

use crate::dbconn::connection;

// TODO instead of unwrapping calls to connection(), check if connection successful,
// return error object over API if not

pub fn get_buildings() -> Vec<Building> {
    let conn = connection().unwrap();
    buildings.load::<Building>(&conn).expect("Error loading buildings")
}

pub fn find_building_by_id(id: uuid::Uuid) -> Option<Building> {
    let conn = connection().unwrap();
    let result = buildings.find(id).first::<Building>(&conn);
    match result {
        Ok(building) => Some(building),
        _            => None
    }
}

pub fn create_or_update_building(id: Option<uuid::Uuid>, building_name: String, building_address: String) -> Option<Building> {
    let conn = connection().unwrap();

    match id {
        Some(uuid) => {
            if let Ok(building) = buildings.find(uuid).get_result::<Building>(&conn) {
                
                diesel::update(&building)
                    .set((b_name.eq(building_name), b_address.eq(building_address)))
                    .get_result(&conn).ok()

            } else {
            
                let new_building = Building {
                    id: uuid,
                    name: building_name,
                    address: building_address
                };

                diesel::insert_into(buildings)
                    .values(new_building)
                    .get_result(&conn).ok()
            }
        },
        None => {
            
            let new_building = Building {
                id: Uuid::new_v4(),
                name: building_name,
                address: building_address
            };
            
            diesel::insert_into(buildings)
                .values(new_building)
                .get_result(&conn).ok()
        
        }
    }
}

pub fn has_storeys(id: uuid::Uuid) -> bool {
    let conn = connection().unwrap();
    match select(exists(storeys.filter(building_id.eq(id)))).get_result(&conn) {
        Ok(has_storeys) => has_storeys,
        Err(_) => false
    }
}

pub fn delete_building_by_id(id: uuid::Uuid) -> bool {
    let conn = connection().unwrap();
    match diesel::delete(buildings.find(id)).execute(&conn) {
        Ok(1) => true,
        _ => false
    }
}

pub fn get_storeys() -> Vec<Storey> {
    let conn = connection().unwrap();
    storeys.load::<Storey>(&conn).expect("Error loading storeys")
}

pub fn find_storey_by_id(id: uuid::Uuid) -> Option<Storey> {
    let conn = connection().unwrap();
    storeys.find(id).get_result::<Storey>(&conn).ok()
}

pub fn create_or_update_storey(id: Option<uuid::Uuid>, storey_name: String, storey_building_id: uuid::Uuid) -> Option<Storey> {
    let conn = connection().unwrap();

    match id {
        Some(uuid) => {

            if let Ok(storey) = storeys.find(uuid).get_result::<Storey>(&conn) {
            
                diesel::update(&storey)
                    .set((s_name.eq(storey_name), building_id.eq(storey_building_id)))
                    .get_result(&conn).ok()

            } else {

                let new_storey = Storey {
                    id: uuid,
                    name: storey_name,
                    building_id: storey_building_id
                };
                
                diesel::insert_into(storeys)
                    .values(new_storey)
                    .get_result(&conn).ok()
            
            }
        },
        None => {
            
            let new_storey = Storey {
                id: Uuid::new_v4(),
                name: storey_name.to_string(),
                building_id: storey_building_id
            };
            
            diesel::insert_into(storeys)
                .values(new_storey)
                .get_result(&conn).ok()
        
        }
    }
}

pub fn has_rooms(id: uuid::Uuid) -> bool {
    let conn = connection().unwrap();
    match select(exists(rooms.filter(storey_id.eq(id)))).get_result(&conn) {
        Ok(has_rooms) => has_rooms,
        Err(_) => false
    }
}

pub fn delete_storey_by_id(id: uuid::Uuid) -> bool {
    let conn = connection().unwrap();
    match diesel::delete(storeys.find(id)).execute(&conn) {
        Ok(1) => true,
        _ => false
    }
}

pub fn get_rooms() -> Vec<Room> {
    let conn = connection().unwrap();
    storeys.load::<Room>(&conn).expect("Error loading storeys")
}

pub fn find_room_by_id(id: uuid::Uuid) -> Option<Room> {
    let conn = connection().unwrap();
    rooms.find(id).get_result::<Room>(&conn).ok()
}

pub fn create_or_update_room(id: Option<uuid::Uuid>, room_name: String, room_storey_id: uuid::Uuid) -> Option<Room> {
    let conn = connection().unwrap();
    
    match id {
        Some(uuid) => {

            if let Ok(room) = rooms.find(uuid).get_result::<Room>(&conn) {
                diesel::update(&room)
                    .set((r_name.eq(room_name), storey_id.eq(room_storey_id)))
                    .get_result(&conn).ok()
            } else {

                let new_room = Room {
                    id: uuid,
                    name: room_name,
                    storey_id: room_storey_id
                };

                diesel::insert_into(rooms)
                    .values(new_room)
                    .get_result(&conn).ok()

            }
        },
        None => {

            let new_room = Room {
                id: Uuid::new_v4(),
                name: room_name,
                storey_id: room_storey_id
            };

            diesel::insert_into(rooms)
                .values(new_room)
                .get_result(&conn).ok()
        
        }
    }
}

pub fn delete_room_by_id(id: uuid::Uuid) -> bool {
    let conn = connection().unwrap();
    match diesel::delete(rooms.find(id)).execute(&conn) {
        Ok(1) => true,
        _ => false
    }
}