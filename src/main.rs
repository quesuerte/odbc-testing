use lazy_static::lazy_static;
use odbc_api::{
    sys::{AttrConnectionPooling, AttrCpMatch},
    Environment,
};
use std::{collections::HashMap, env, process, sync::mpsc, thread};
mod catalog;
use crate::catalog::*;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

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

fn main() {
    let env_var: Vec<&str> = vec![DB_USER, DB_PASS, DB_URL];
    let var_map = match fill_environment_variables(&env_var) {
        Ok(val) => val,
        Err(e) => {
            eprintln!(
                "{e}: the following environment variables must be set:
                {:#?}",
                env_var
            );
            process::exit(1);
        }
    };

    let env = match Environment::new() {
        Ok(env) => env,
        Err(e) => {
            eprintln!("Could not connect to the ODBC environment: {e}");
            process::exit(1);
        }
    };

    let pool = get_connection_pool(var_map);
    let (tx, rx) = mpsc::channel();
    tx.send(pool).unwrap();
    let mut threads = vec![];
    let max_users_to_create = 1;

    for i in 0..max_users_to_create {
        let mut conn = env.connect(
            "YourDatabase",
            "SA",
            "My@Test@Password1",
            ConnectionOptions::default(),
        )?;
        /*let pool: Pool<ConnectionManager<PgConnection>> = rx.recv().unwrap();
        let tx_thread = tx.clone();
        threads.push(thread::spawn({
            move || {
                let mut catalog = Catalog::new(pool.get().unwrap());
                tx_thread.send(pool).unwrap();
                catalog.add_database("postgres","postgres","admin","PostgresDSN").unwrap();
            }
        }));
        */
    }

    for handle in threads {
        handle.join().unwrap();
    }
}

fn fill_environment_variables<'a>(
    in_vars: &'a Vec<&str>,
) -> Result<HashMap<&'a str, String>, env::VarError> {
    let mut ret_map: HashMap<&str, String> = HashMap::new();
    for key in in_vars.iter() {
        match env::var(key) {
            Ok(val) => match ret_map.insert(key, val) {
                Some(_) => {
                    panic!("env var hashmap should be empty--two of the same value are present")
                }
                None => (),
            },
            Err(e) => return Err(e),
        }
    }
    return Ok(ret_map);
}
