use super::schema::units;

#[derive(Serialize,Deserialize,Queryable,Identifiable)]
#[belongs_to(Food)]
#[table_name="units"]
pub struct Unit {
    pub id: i32,
    pub name: String,
    pub grams: f32,

    pub food: Option<i32>,
}

#[derive(Serialize,Deserialize,Insertable)]
#[table_name="units"]
pub struct NewUnit {
    pub name: String,
    pub grams: f32,
    pub food: Option<i32>,
}
