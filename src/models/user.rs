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

impl User {
    pub fn verify_password(&self, password: &String) -> bool {
        bcrypt::verify(password.as_str(), self.password.as_str()).unwrap()
    }

    pub fn to_display_user(&self) -> DisplayUser {
        DisplayUser {
            id: self.id,
            name: self.name.clone(),
            username: self.name.clone(),
        }
    }
}

#[derive(Debug, Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct DisplayUser {
    pub id: i32,
    pub name: String,
    pub username: String,
}
