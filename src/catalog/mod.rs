mod backend_connection;
mod elements;
mod metadata_schema;
use elements::{Column, Database, Schema, Table};
use metadata_schema::*;


use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::result::Error;
use std::collections::HashMap;
use diesel::r2d2::PooledConnection;

pub const DB_USER: &str = "DB_USER";
pub const DB_PASS: &str = "DB_PASS";
pub const DB_URL: &str = "DB_URL";

pub fn get_connection_pool(
    env_var: HashMap<&str, String>,
) -> Pool<ConnectionManager<PgConnection>> {
    let url = format!(
        "postgres://{}:{}@{}",
        env_var.get(DB_USER).expect("env_vars should have value"),
        env_var.get(DB_PASS).expect("env_vars should have value"),
        env_var.get(DB_URL).expect("env_vars should have value"),
    );
    let manager = ConnectionManager::<PgConnection>::new(url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder().test_on_check_out(true).build(manager).expect("could not build pool")
}

pub struct Catalog {
    conn: PooledConnection<ConnectionManager<PgConnection>>,
}

impl Catalog {
    pub fn new(conn: PooledConnection<ConnectionManager<PgConnection>>) -> Self {
        Catalog { conn }
    }
    pub fn add_database(&mut self, name: &str, user: &str, pass: &str, uri: &str) -> Result<(), Error> {
        match databases::table
            .filter(databases::name.eq(name))
            .select(Database::as_select())
            .first(&mut self.conn)
        {
            Ok(_) => return Err(diesel::result::Error::NotFound),
            Err(_) => (),
        };
        match diesel::insert_into(databases::table)
            .values((
                databases::name.eq(name),
                databases::uri.eq(uri),
                databases::username.eq(user),
                databases::pass.eq(pass),
            ))
            .execute(&mut self.conn)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    pub fn all_schemas(&mut self) -> Result<Vec<Schema>, Error> {
        let all_dbs = databases::table
            .select(Database::as_select())
            .load(&mut self.conn)?;
        let all_schema = Schema::belonging_to(&all_dbs)
            .select(Schema::as_select())
            .load::<Schema>(&mut self.conn)?;
        Ok(all_schema)
    }
    pub fn get_schema_by_name(&mut self, database: &str, name: &str) -> Result<i32, Error> {
        let matching_dbs = databases::table
            .filter(databases::name.eq(database))
            .select(Database::as_select())
            .load(&mut self.conn)?;
        let schema = Schema::belonging_to(&matching_dbs)
            .filter(schemas::name.eq(name))
            .select(Schema::as_select())
            .first::<Schema>(&mut self.conn)?;
        Ok(schema.id)
    }
}
