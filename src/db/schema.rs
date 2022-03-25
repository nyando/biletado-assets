use diesel::table;

// This module defines the tables and their column types in the `assets` database.
// The `table!` macro generates ORM query methods for the structures within.

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
