diesel::table! {
    databases (id) {
        id -> Int4,
        name -> Text,
        uri -> Text,
        username -> Text,
        pass -> Text,
    }
}

diesel::table! {
    schemas (id) {
        database_id -> Int4,
        id -> Int4,
        name -> Text,
        table_id -> Int4,
    }
}

diesel::table! {
    tables (id) {
        schema_id -> Int4,
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    columns (id) {
        table_id -> Int4,
        id -> Int4,
        name -> Text,
    }
}

diesel::joinable!(databases -> schemas (id));
diesel::joinable!(schemas -> tables (id));
diesel::joinable!(tables -> columns (id));

diesel::allow_tables_to_appear_in_same_query!(
    databases,
    schemas,
    tables,
    columns
);