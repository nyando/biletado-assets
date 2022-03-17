use diesel::table;

table! {
    pub buildings (id) {
        id -> diesel::sql_types::Uuid,
        name -> Text,
        address -> Text,
    }
}

table! {
    pub storeys (id) {
        id -> diesel::sql_types::Uuid,
        name -> Text,
        building_id -> diesel::sql_types::Uuid,
    }
}

table! {
    pub rooms (id) {
        id -> diesel::sql_types::Uuid,
        name -> Text,
        storey_id -> diesel::sql_types::Uuid,
    }
}
