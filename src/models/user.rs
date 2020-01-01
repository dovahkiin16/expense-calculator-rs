use chrono::NaiveDateTime;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime
}