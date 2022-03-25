use diesel::{Queryable, Insertable, Identifiable};
use serde::{Serialize, Deserialize};
use crate::db::schema::*;

#[derive(Serialize, Deserialize, Queryable, Insertable, Identifiable)]
/// Building type, identified by UUID, has a name and an address.
pub struct Building {
    pub id: uuid::Uuid,
    pub name: String,
    pub address: String
}

#[derive(Deserialize)]
/// Building type, potentially without UUID, that may be passed as part of a POST or PUT request.
pub struct OptionalIDBuilding {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub address: String
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Identifiable)]
/// Storey type, identified by UUID, has an associated building and a name.
pub struct Storey {
    pub id: uuid::Uuid,
    pub name: String,
    pub building_id: uuid::Uuid
}

#[derive(Deserialize)]
/// Storey type, potentially without UUID, that may be passed as part of a POST or PUT request.
pub struct OptionalIDStorey {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub building_id: uuid::Uuid
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Identifiable)]
/// Room type, identified by UUID, has an associated storey and a name.
pub struct Room {
    pub id: uuid::Uuid,
    pub name: String,
    pub storey_id: uuid::Uuid
}

#[derive(Deserialize)]
/// Room type, potentially without UUID, that may be passed as part of a POST or PUT request.
pub struct OptionalIDRoom {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub storey_id: uuid::Uuid
}

#[derive(Deserialize)]
/// Reservation type, used to check for existing room reservations while deleting rooms.
pub struct Reservation {
    pub id: uuid::Uuid,
    pub from: String,
    pub to: String,
    pub room_id: uuid::Uuid
}
