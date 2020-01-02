use crate::schema::users;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Identifiable, Associations, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
}
