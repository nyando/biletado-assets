use crate::diesel::{RunQueryDsl, QueryDsl};
use crate::db::models::*;
use crate::db::schema::buildings::dsl::buildings;

use crate::dbconn::connection;

// TODO instead of unwrapping calls to connection(), check if connection successful,
// return error object over API if not

pub fn get_buildings() -> Vec<Building> {
    let conn = connection().unwrap();
    buildings.load::<Building>(&conn).expect("Error loading buildings")
}

pub fn find_building_by_id(building_id: uuid::Uuid) -> Option<Building> {
    let conn = connection().unwrap();
    let result = buildings.find(building_id).first::<Building>(&conn);
    match result {
        Ok(building) => Some(building),
        _            => None
    }
}