use chrono::NaiveDateTime;
use serde::Serialize;
use crate::schema::users;

#[derive(Debug, Identifiable, Associations, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime
}
