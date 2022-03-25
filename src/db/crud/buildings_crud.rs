use uuid::Uuid;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::db::models::Building;
use crate::db::schema::buildings::dsl::buildings;
use crate::db::schema::buildings::name as b_name;
use crate::db::schema::buildings::address as b_address;

use crate::dbconn::connection;

/// Get a vector of all buildings in the database.
pub fn get_buildings() -> Vec<Building> {
    let conn = connection().unwrap();
    buildings.load::<Building>(&conn).expect("Error loading buildings")
}

/// Find a building by UUID.
/// Returns a building struct with the corresponding UUID or None if the UUID is not in the DB.
pub fn find_building_by_id(id: uuid::Uuid) -> Option<Building> {
    let conn = connection().unwrap();
    buildings.find(id).first::<Building>(&conn).ok()
}

/// Pass a building name and address, maybe a UUID.
/// If the UUID already exists, update the building with the new name and address.
/// If the UUID does not exist, create a new building with that UUID.
/// If there is no UUID, generate a new one and insert a new building with that name, address, and new UUID.
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

/// Delete the building with the UUID id.
/// Return true if deletion was successful, false if the UUID was not found.
pub fn delete_building_by_id(id: uuid::Uuid) -> bool {
    let conn = connection().unwrap();
    matches!(diesel::delete(buildings.find(id)).execute(&conn), Ok(1))
}