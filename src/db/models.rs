extern crate serde;

use diesel::Queryable;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct Building {
    pub id: uuid::Uuid,
    pub name: String,
    pub address: String
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Storey {
    pub id: uuid::Uuid,
    pub building_id: uuid::Uuid,
    pub name: String
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Room {
    pub id: uuid::Uuid,
    pub name: String,
    pub storey_id: uuid::Uuid
}
