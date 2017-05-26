#![feature(custom_attribute,plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate lazy_static;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

pub mod models;

use diesel::sqlite::SqliteConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use rocket_contrib::JSON;

lazy_static! {
    pub static ref CONN_POOL: Pool<ConnectionManager<SqliteConnection>> = models::create_conn_pool();
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/foods")]
fn foods() -> JSON<Vec<models::Food>> {
    use diesel::LoadDsl;
    use models::schema::foods;

    let conn = CONN_POOL.get().unwrap();

    let foods = foods::table.load::<models::food::Food>(&*conn).unwrap();

    JSON(foods)
}

#[post("/foods", data="<food>")]
fn create_food(food: JSON<models::NewFood>) {
    use diesel::ExecuteDsl;
    use models::schema::foods;

    let conn = CONN_POOL.get().unwrap();
    diesel::insert(&food.0).into(foods::table).execute(&*conn).unwrap();
}

#[get("/units")]
fn units() -> JSON<Vec<models::Unit>> {
    use diesel::LoadDsl;
    use models::schema::units;

    let conn = CONN_POOL.get().unwrap();

    let units = units::table.load::<models::units::Unit>(&*conn).unwrap();

    JSON(units)
}

#[post("/units", data="<unit>")]
fn create_unit(unit: JSON<models::NewUnit>) {
    use diesel::ExecuteDsl;
    use models::schema::units;

    let conn = CONN_POOL.get().unwrap();
    diesel::insert(&unit.0).into(units::table).execute(&*conn).unwrap();
}

fn main() {
    rocket::ignite().mount("/", routes![index, foods, create_food, units, create_unit]).launch();
}
