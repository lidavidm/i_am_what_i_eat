use super::schema::foods;

#[derive(Serialize,Deserialize,Queryable)]
pub struct Food {
    pub id: i32,
    pub name: String,

    pub calories: f32,
    pub lipids: f32,
    pub protein: f32,
    pub carbohydrates: f32,
}

#[derive(Serialize,Deserialize,Insertable)]
#[table_name="foods"]
pub struct NewFood {
    pub name: String,

    pub calories: f32,
    pub lipids: f32,
    pub protein: f32,
    pub carbohydrates: f32,
}
