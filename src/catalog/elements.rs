use diesel::prelude::*;
use super::metadata_schema::{databases,schemas,tables,columns};

#[derive(Identifiable,Queryable, Selectable, PartialEq, Debug)]
#[diesel(table_name = databases)]
pub struct Database {
    pub id: i32,
    pub name: String,
    uri: String,
    username: String,
    pass: String,
}

#[derive(Identifiable,Queryable,Associations, Selectable, PartialEq, Debug)]
#[diesel(belongs_to(Database))]
#[diesel(table_name = schemas)]
pub struct Schema {
    pub database_id: i32,
    pub id: i32,
    pub name: String,
    pub table_id: i32,
}

#[derive(Identifiable,Queryable,Associations, Selectable, PartialEq, Debug)]
#[diesel(belongs_to(Schema))]
#[diesel(table_name = tables)]
pub struct Table {
    pub schema_id: i32,
    pub id: i32,
    pub name: String,
}

#[derive(Identifiable,Queryable,Associations, Selectable, PartialEq, Debug)]
#[diesel(belongs_to(Table))]
#[diesel(table_name = columns)]
pub struct Column {
    pub table_id: i32,
    pub id: i32,
    pub name: String,
}