use chrono;

use super::schema::entries;

#[derive(Serialize,Deserialize,Queryable,Identifiable)]
#[table_name="entries"]
pub struct Entry {
    pub id: i32,
    pub date: chrono::NaiveDate,

    pub food: i32,
    pub unit: Option<i32>,
    pub quantity: f32,
}

#[derive(Serialize,Deserialize,Insertable)]
#[table_name="entries"]
pub struct NewEntry {
    pub date: chrono::NaiveDate,

    pub food: i32,
    pub unit: Option<i32>,
    pub quantity: f32,
}
