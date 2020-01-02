use crate::diesel::prelude::*;
use crate::models::{Expense, NewExpense};
use diesel::result::Error;

pub fn find_all(user_id: i32) -> Vec<Expense> {
    use crate::schema::expenses;
    // Get connection to db
    let conn = crate::establish_connection();

    expenses::table
        .filter(expenses::user_id.eq(user_id))
        .order(expenses::created_at)
        .load::<Expense>(&conn)
        .expect("Error loading expenses") // TODO: handle error properly
}

pub fn add_one(item: NewExpense) -> Result<Expense, Error> {
    use crate::schema::expenses;
    //Get connection to db
    let conn = crate::establish_connection();

    diesel::insert_into(expenses::table)
        .values(&item)
        .get_result::<Expense>(&conn)
}
