use chrono::NaiveDateTime;

#[derive(Debug, Identifiable, Associations, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime
}