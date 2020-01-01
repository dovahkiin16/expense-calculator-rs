use crate::models::User;
use crate::diesel::prelude::*;

pub fn find_all() -> Vec<User> {
    use crate::schema::users;
    // Get connection to db
    let conn = crate::establish_connection();

    users::table.order(users::created_at)
        .load::<User>(&conn)
        .expect("Error loading users") // TODO: handle this error
}