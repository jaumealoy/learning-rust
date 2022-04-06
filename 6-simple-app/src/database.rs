use mysql_async::prelude::Queryable;
use mysql_async::{Opts, OptsBuilder, Pool, Conn};
use core::panic;
use std::fs::{File};
use std::io::Read;
use std::path::Path;
use lazy_static::lazy_static;
use thiserror::Error;

lazy_static! {
    static ref POOL: Pool = {
        let hostname = std::env::var("DB_HOST").unwrap_or("".to_owned());
        let username = std::env::var("DB_USER").unwrap_or("".to_owned());
        let password = std::env::var("DB_PASS").unwrap_or("".to_owned());
        let db_name = std::env::var("DB_NAME").unwrap_or("".to_owned());

        println!("Creating pool?");

        let connection_options: Opts = OptsBuilder::default()
            .ip_or_hostname(hostname)
            .user(Some(username))
            .pass(Some(password))
            .db_name(Some(db_name))
            .into();

        Pool::new(connection_options)
    };
}

pub async fn init() {
    let connection = (*POOL).clone();
    let result = connection.get_conn().await;

    if let Err(x) = result {
        panic!("Error establising connection with MySQL. Details: {:?}", x);
    }

    let filename = Path::new("data/db.sql");
    let mut file = File::open(filename);
    if let Ok(fd) = &mut file {
        let mut buffer = String::new();

        if let Ok(_) = fd.read_to_string(&mut buffer) {
            let mut con = connection.get_conn().await.unwrap();
            con.query_drop(buffer).await;
        } else {
            panic!("Error reading database file.");
        }
    } else {
        panic!("Error creating database.");
    }
}

#[derive(Error, Debug)]
#[error("Database error. See internal_error for more details.")]
pub struct DBError {
    internal_error: mysql_async::Error
}

pub async fn get_connection() -> Result<Conn, Box<dyn std::error::Error>> {
    let pool = (*POOL).clone();

    match pool.get_conn().await {
        Ok(connection) => Ok(connection),
        Err(x) => Err(Box::new(DBError{ internal_error: x }))
    }
}

pub async fn close() {
    let pool = (*POOL).clone();
    pool.disconnect().await;
}