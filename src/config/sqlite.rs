use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

pub fn new_sqlite_connection() -> SqliteConnection {
    let db_url = env::var("DATABASE_URL").expect("missing sqlite db_url");

    SqliteConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
