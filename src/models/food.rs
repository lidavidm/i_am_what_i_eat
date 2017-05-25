#[derive(Queryable)]
pub struct Food {
    pub id: i32,
    pub name: String,

    pub calories: f32,
}
