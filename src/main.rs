use std::{env::{self, VarError}, process, thread};
mod catalog;
use crate::catalog::*;

pub const DB_USER: &str = "DB_USER";
pub const DB_PASS: &str = "DB_PASS";
pub const DB_DSN: &str = "DB_DSN";
fn main() {
    let db_dsn = match env::var(DB_DSN) {
        Ok(val) => val,
        Err(e) => {
            missing_env_variables(e);
            process::exit(1);
        }
    };
    let db_user = match env::var(DB_USER) {
        Ok(val) => val,
        Err(e) => {
            missing_env_variables(e);
            process::exit(1);
        }
    };
    let db_pass = match env::var(DB_PASS) {
        Ok(val) => val,
        Err(e) => {
            missing_env_variables(e);
            process::exit(1);
        }
    };

    

    let mut threads = vec![];
    let max_users_to_create = 1;

    for i in 0..max_users_to_create {
        let thread_dsn = db_dsn.clone();
        let thread_user = db_user.clone();
        let thread_pass = db_pass.clone();
        threads.push(thread::spawn({
            move || {
                match Catalog::build(&thread_dsn,&thread_user,&thread_pass) {
                    Ok(catalog) => println!("{:?}",catalog.get_databases().unwrap()),
                    Err(e) => eprintln!("Thread {i} Error connecting to metadata catalog: {e}"),
                }
            }
        }));
    }

    for handle in threads {
        handle.join().unwrap();
    }
}

fn missing_env_variables(e: VarError) {
    eprintln!(
        "{e}: the following environment variables must be set:
        `DB_USER`
        `DB_PASS`
        `DB_DSN`"
    );
}
