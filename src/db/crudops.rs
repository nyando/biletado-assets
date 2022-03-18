use uuid::Uuid;

use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::select;
use diesel::dsl::exists;

use crate::db::models::*;
use crate::db::schema::buildings::dsl::buildings;
use crate::db::schema::buildings::name;
use crate::db::schema::buildings::address;

use crate::db::schema::storeys::dsl::storeys;
use crate::db::schema::storeys::building_id;

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
            let result = diesel::update(buildings.find(uuid))
                .set((name.eq(building_name), address.eq(building_address)))
                .get_result(&conn);
            match result {
                Ok(building) => Some(building),
                Err(_) => None
            }
        },
        None => {
            let new_uuid = Uuid::new_v4();
            let new_building = Building {
                id: new_uuid,
                name: building_name,
                address: building_address
            };
            let result = diesel::insert_into(buildings)
                .values(new_building)
                .get_result(&conn);
            match result {
                Ok(building) => Some(building),
                Err(_) => None
            }
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