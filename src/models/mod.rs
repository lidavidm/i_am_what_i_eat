use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub mod food;

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("test.db").unwrap()
}

pub mod schema {
    infer_schema!("test.db");
}
