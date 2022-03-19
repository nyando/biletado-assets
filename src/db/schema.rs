use diesel::table;

table! {
    pub buildings (id) {
        id -> diesel::sql_types::Uuid,
        name -> diesel::sql_types::Text,
        address -> diesel::sql_types::Text,
    }
}

table! {
    pub storeys (id) {
        id -> diesel::sql_types::Uuid,
        name -> diesel::sql_types::Text,
        building_id -> diesel::sql_types::Uuid,
    }
}

table! {
    pub rooms (id) {
        id -> diesel::sql_types::Uuid,
        name -> diesel::sql_types::Text,
        storey_id -> diesel::sql_types::Uuid,
    }
}
