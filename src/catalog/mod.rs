use lazy_static::lazy_static;
use odbc_api::{
    buffers::ColumnarAnyBuffer,
    Connection, ConnectionOptions, Cursor, Environment, Error,
    sys::{AttrConnectionPooling, AttrCpMatch}
};
use odbc_api::buffers::BufferDesc;
lazy_static! {
    pub static ref ENV: Environment = {
        // Enable connection pooling. Let driver decide whether the attributes of two connection
        // are similar enough to change the attributes of a pooled one, to fit the requested
        // connection, or if it is cheaper to create a new Connection from scratch.
        // See <https://docs.microsoft.com/en-us/sql/odbc/reference/develop-app/driver-aware-connection-pooling>
        //
        // Safety: This call changes global mutable space in the underlying ODBC driver manager.
        unsafe {
            Environment::set_connection_pooling(AttrConnectionPooling::DriverAware).unwrap();
        }
        let mut env = Environment::new().unwrap();
        // Strict is the default, and is set here to be explicit about it.
        env.set_connection_pooling_matching(AttrCpMatch::Strict).unwrap();
        env
    };
}

const BATCH_SIZE: usize = 1000; // Maximum number of rows in each row set

pub struct Catalog<'a> {
    conn: Connection<'a>,
}

impl<'a> Catalog<'a> {
    pub fn build(db_dsn: &str, db_user: &str, db_pass: &str) -> Result<Self,Error> {
        Ok(Catalog { conn: ENV.connect(
            db_dsn,
            db_user,
            db_pass,
            ConnectionOptions::default(),
        )? })
    }
    pub fn get_databases(&self) -> Result<Vec<Database>, Error> {
        let mut ret_db: Vec<Database> = Vec::new();
        let buffer_desc = [
            // We know year to be a Nullable SMALLINT
            BufferDesc::I8 {nullable: false},
            // and name to be a required VARCHAR
            BufferDesc::Text { max_str_len: 32 },
            BufferDesc::Text { max_str_len: 32 },
            BufferDesc::Text { max_str_len: 32 },
            BufferDesc::Text { max_str_len: 32 },
        ];
        // Creates a columnar buffer fitting the buffer description with the capacity of `batch_size`.
        let mut buffer = ColumnarAnyBuffer::from_descs(BATCH_SIZE, buffer_desc);
        match self
            .conn
            .execute("SELECT id, name, uri, username, pass FROM databases", ())?
        {
            Some(cursor) => {
                let mut row_set_cursor = cursor.bind_buffer(&mut buffer)?;
                // Loop over row sets
                while let Some(row_set) = row_set_cursor.fetch()? {
                    // These columns are NOT NULL
                    let name_col = row_set.column(1).as_text_view().unwrap();
                    let uri_col = row_set.column(2).as_text_view().unwrap();
                    let username_col = row_set.column(3).as_text_view().unwrap();
                    let pass_col = row_set.column(4).as_text_view().unwrap();
                    for i in 0..(row_set.num_rows()-1) {
                        ret_db.push(Database {
                            id: row_set.column(0).as_slice().unwrap()[i],
                            name: match String::from_utf8(name_col.get(i).unwrap().to_owned()) {
                                Ok(val) => val,
                                Err(e) => e.to_string(),
                            },
                            uri: match String::from_utf8(uri_col.get(i).unwrap().to_owned()) {
                                Ok(val) => val,
                                Err(e) => e.to_string(),
                            },
                            username: match String::from_utf8(username_col.get(i).unwrap().to_owned()) {
                                Ok(val) => val,
                                Err(e) => e.to_string(),
                            },
                            pass: match String::from_utf8(pass_col.get(i).unwrap().to_owned()) {
                                Ok(val) => val,
                                Err(e) => e.to_string(),
                            },
                        });
                    }
                }
                Ok(ret_db)
            }
            None => Err(odbc_api::Error::NoDiagnostics {
                function: "query did not return a cursor",
            }),
        }
    }
}

#[derive(Debug)]
pub struct Database {
    pub id: i32,
    pub name: String,
    uri: String,
    username: String,
    pass: String,
}

pub struct Schema {
    pub database_id: i32,
    pub id: i32,
    pub name: String,
    pub table_id: i32,
}

pub struct Table {
    pub schema_id: i32,
    pub id: i32,
    pub name: String,
}

pub struct Column {
    pub table_id: i32,
    pub id: i32,
    pub name: String,
}
