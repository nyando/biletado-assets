use diesel::Queryable;

#[derive(Queryable)]
pub struct Building {
    pub id: uuid::Uuid,
    pub name: String,
    pub address: String
}

#[derive(Queryable)]
pub struct Storey {
    pub id: uuid::Uuid,
    pub building_id: uuid::Uuid,
    pub name: String
}

#[derive(Queryable)]
pub struct Room {
    pub id: uuid::Uuid,
    pub name: String,
    pub storey_id: uuid::Uuid
}
