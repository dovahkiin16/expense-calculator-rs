use crate::diesel::prelude::*;
use crate::models::{Expense, NewExpense};
use diesel::dsl::sum;
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

pub fn get_expenses_sum(user_id: i32) -> Option<f32> {
    use crate::schema::expenses;

    let conn = crate::establish_connection();
    expenses::table
        .select(sum(expenses::amount))
        .filter(expenses::user_id.eq(user_id))
        .first(&conn)
        .unwrap()
}
