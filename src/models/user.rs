use crate::schema::users;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Identifiable, Associations, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub username: String,
    pub password: String,
}
