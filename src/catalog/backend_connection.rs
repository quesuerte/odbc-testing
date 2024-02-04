use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::collections::HashMap;
use super::*;
use odbc_api::{
    Environment, Cursor, ConnectionOptions,
    buffers::{AnySlice, BufferDesc, Item, ColumnarAnyBuffer},
};

const BATCH_SIZE: u8 = 1000; // Maximum number of rows in each row set

pub fn establish_connection(conn_var: HashMap<&str,String>) -> Result<PgConnection,ConnectionError> {
    let database_url = format!("postgres://{}:{}@{}",
        conn_var.get(DB_USER).expect("parsed environment variables should not be empty"),
        conn_var.get(DB_PASS).expect("parsed environment variables should not be empty"),
        conn_var.get(DB_URL).expect("parsed environment variables should not be empty"),
    );
    PgConnection::establish(&database_url)
}

pub fn query_databases(meta_conn: Connection) {

}