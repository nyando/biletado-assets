use uuid::Uuid;

use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::db::models::Room;
use crate::db::schema::rooms::dsl::rooms;
use crate::db::schema::rooms::storey_id;
use crate::db::schema::rooms::name as r_name;

use crate::dbconn::connection;

pub fn has_rooms(id: uuid::Uuid) -> bool {
    let conn = connection().unwrap();
    match diesel::select(diesel::dsl::exists(rooms.filter(storey_id.eq(id)))).get_result(&conn) {
        Ok(has_rooms) => has_rooms,
        Err(_) => false
    }
}

pub fn get_rooms() -> Vec<Room> {
    let conn = connection().unwrap();
    rooms.load::<Room>(&conn).expect("Error loading storeys")
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