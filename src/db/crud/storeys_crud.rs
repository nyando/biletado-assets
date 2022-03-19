use uuid::Uuid;

use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::db::models::*;

use crate::db::schema::storeys::dsl::storeys;
use crate::db::schema::storeys::building_id;
use crate::db::schema::storeys::name as s_name;

use crate::dbconn::connection;

// TODO instead of unwrapping calls to connection(), check if connection successful,
// return error object over API if not

pub fn has_storeys(id: uuid::Uuid) -> bool {
    let conn = connection().unwrap();
    match diesel::select(diesel::dsl::exists(storeys.filter(building_id.eq(id)))).get_result(&conn) {
        Ok(has_storeys) => has_storeys,
        Err(_) => false
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

pub fn delete_storey_by_id(id: uuid::Uuid) -> bool {
    let conn = connection().unwrap();
    match diesel::delete(storeys.find(id)).execute(&conn) {
        Ok(1) => true,
        _ => false
    }
}