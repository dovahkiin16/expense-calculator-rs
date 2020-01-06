use crate::diesel::prelude::*;
use crate::models::{NewUser, User};
use diesel::result::Error;

pub fn find_all() -> Vec<User> {
    use crate::schema::users;
    // Get connection to db
    let conn = crate::establish_connection();

    users::table
        .order(users::created_at)
        .load::<User>(&conn)
        .expect("Error loading users") // TODO: handle this error
}

pub fn find_one_by_username(username: &String) -> Option<User> {
    use crate::schema::users;
    // Get connection to db
    let conn = crate::establish_connection();

    let user_res: Result<User, Error> = users::table
        .filter(users::username.eq(username))
        .get_result::<User>(&conn);

    match user_res {
        Ok(user) => Some(user),
        Err(e) => {
            dbg!{e};
            None
        },
    }
}

pub fn add_one(item: NewUser) -> Result<User, Error> {
    use crate::schema::users;
    // Get connection to db
    let conn = crate::establish_connection();

    diesel::insert_into(users::table)
        .values(&item)
        .get_result::<User>(&conn)
}
