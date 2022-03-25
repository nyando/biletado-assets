use uuid::Uuid;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::db::models::*;

use crate::db::schema::storeys::dsl::storeys;
use crate::db::schema::storeys::building_id;
use crate::db::schema::storeys::name as s_name;

use crate::dbconn::connection;

/// Check if a building has associated storeys.
/// Return true if a building has associated storeys, false otherwise.
pub fn has_storeys(id: uuid::Uuid) -> bool {
    let conn = connection().unwrap();
    diesel::select(diesel::dsl::exists(storeys.filter(building_id.eq(id))))
        .get_result(&conn)
        .unwrap_or(false)
}

/// Return a vector of all storeys in the database.
pub fn get_storeys() -> Vec<Storey> {
    let conn = connection().unwrap();
    storeys.load::<Storey>(&conn).expect("Error loading storeys")
}

/// Find a storey by UUID.
/// Returns a storey struct with the corresponding UUID or None if the UUID is not in the DB.
pub fn find_storey_by_id(id: uuid::Uuid) -> Option<Storey> {
    let conn = connection().unwrap();
    storeys.find(id).get_result::<Storey>(&conn).ok()
}

/// Pass a storey name and building ID, maybe a room UUID.
/// If the UUID already exists, update the storey with the new name and building ID.
/// If the UUID does not exist, create a new storey with that UUID.
/// If there is no UUID, generate a new one and insert it with that name and building ID.
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
                name: storey_name,
                building_id: storey_building_id
            };
            
            diesel::insert_into(storeys)
                .values(new_storey)
                .get_result(&conn).ok()
        
        }
    }
}

/// Delete the storey with the UUID id.
/// Return true if deletion was successful, false if the UUID was not found.
pub fn delete_storey_by_id(id: uuid::Uuid) -> bool {
    let conn = connection().unwrap();
    matches!(diesel::delete(storeys.find(id)).execute(&conn), Ok(1))
}