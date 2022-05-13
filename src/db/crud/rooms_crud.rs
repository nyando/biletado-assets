use uuid::Uuid;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::db::models::Room;
use crate::db::schema::rooms::dsl::rooms;
use crate::db::schema::rooms::storey_id;
use crate::db::schema::rooms::name as r_name;

use crate::dbconn::connection;

/// Check if a storey has associated rooms.
/// Return true if a storey has associated rooms, false otherwise.
pub fn has_rooms(id: uuid::Uuid) -> bool {
    let conn = connection().unwrap();
    diesel::select(diesel::dsl::exists(rooms.filter(storey_id.eq(id))))
        .get_result(&conn)
        .unwrap_or(false)
}

/// Return a vector of all rooms in the database.
pub fn get_rooms() -> Vec<Room> {
    let conn = connection().unwrap();
    rooms.load::<Room>(&conn).expect("Error loading storeys")
}

pub fn rooms_by_storey(id: uuid::Uuid) -> Vec<Room> {
    let conn = connection().unwrap();
    rooms.filter(storey_id.eq(id)).load::<Room>(&conn).unwrap_or(Vec::new())
}

/// Find a room by UUID.
/// Returns a room struct with the corresponding UUID or None if the UUID is not in the DB.
pub fn find_room_by_id(id: uuid::Uuid) -> Option<Room> {
    let conn = connection().unwrap();
    rooms.find(id).get_result::<Room>(&conn).ok()
}

/// Pass a room name and storey ID, maybe a room UUID.
/// If the UUID already exists, update the room with the new name and storey UUID.
/// If the UUID does not exist, create a new room with that UUID.
/// If there is no UUID, generate a new one and insert it with that name and storey ID.
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

/// Delete the room with the UUID id.
/// Return true if deletion was successful, false if the UUID was not found.
pub fn delete_room_by_id(id: uuid::Uuid) -> bool {
    let conn = connection().unwrap();
    matches!(diesel::delete(rooms.find(id)).execute(&conn), Ok(1))
}