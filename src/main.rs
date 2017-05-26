#![feature(plugin)]
#![plugin(rocket_codegen)]

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
fn foods() -> String {
    use diesel::LoadDsl;
    use models::schema::foods;

    let conn = CONN_POOL.get().unwrap();

    let foods = foods::table.load::<models::food::Food>(&*conn).unwrap();

    format!("Foods: {}", foods.len())
}

#[post("/foods", data="<food>")]
fn create_food(food: JSON<models::NewFood>) {
    use diesel::ExecuteDsl;
    use models::schema::foods;

    let conn = CONN_POOL.get().unwrap();
    diesel::insert(&food.0).into(foods::table).execute(&*conn).unwrap();
}

fn main() {
    rocket::ignite().mount("/", routes![index, foods, create_food]).launch();
}
