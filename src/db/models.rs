use diesel::{Queryable, Insertable, Identifiable};
use serde::{Serialize, Deserialize};
use crate::db::schema::*;

#[derive(Serialize, Deserialize, Queryable, Insertable, Identifiable)]
pub struct Building {
    pub id: uuid::Uuid,
    pub name: String,
    pub address: String
}

#[derive(Deserialize)]
pub struct OptionalIDBuilding {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub address: String
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Identifiable)]
pub struct Storey {
    pub id: uuid::Uuid,
    pub name: String,
    pub building_id: uuid::Uuid
}

#[derive(Deserialize)]
pub struct OptionalIDStorey {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub building_id: uuid::Uuid
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Identifiable)]
pub struct Room {
    pub id: uuid::Uuid,
    pub name: String,
    pub storey_id: uuid::Uuid
}

#[derive(Deserialize)]
pub struct OptionalIDRoom {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub storey_id: uuid::Uuid
}

#[derive(Deserialize)]
pub struct Reservation {
    pub id: uuid::Uuid,
    pub from: String,
    pub to: String,
    pub room_id: uuid::Uuid
}
