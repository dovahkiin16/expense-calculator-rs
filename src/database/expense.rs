use crate::models::Expense;
use crate::diesel::prelude::*;

pub fn find_all(user_id: i32) -> Vec<Expense> {
    use crate::schema::expenses;
    // Get connection to db
    let conn = crate::establish_connection();

    expenses::table.filter(expenses::user_id.eq(user_id))
        .order(expenses::created_at)
        .load::<Expense>(&conn)
        .expect("Error loading expenses") // TODO: handle error properly
}