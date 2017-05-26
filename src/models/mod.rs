use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use r2d2::{Config, Pool};
use r2d2_diesel::ConnectionManager;

pub mod food;

pub use self::food::{Food, NewFood};

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("test.db").unwrap()
}

pub fn create_conn_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    let config = Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new("test.db");
    Pool::new(config, manager).unwrap()
}

pub mod schema {
    infer_schema!("test.db");
}
