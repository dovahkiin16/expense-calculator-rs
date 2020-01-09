use crate::diesel::prelude::*;
use crate::models::{Expense, NewExpense};
use chrono::NaiveDateTime;
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

pub fn get_expenses_sum(
    user_id: i32,
    exp_type: Option<String>,
    from: Option<NaiveDateTime>,
    to: Option<NaiveDateTime>,
    need: Option<bool>,
) -> Option<f32> {
    use crate::schema::expenses;

    let conn = crate::establish_connection();
    let mut query = expenses::table
        .into_boxed()
        .select(sum(expenses::amount))
        .filter(expenses::user_id.eq(user_id));

    // expense type filter
    if exp_type.is_some() {
        let exp_type = exp_type.unwrap();
        query = query.filter(expenses::expense_type.eq(exp_type));
    };

    // date filter
    if from.is_some() {
        query = query.filter(expenses::created_at.ge(from.unwrap()));
    };
    if to.is_some() {
        query = query.filter(expenses::created_at.lt(to.unwrap()));
    };

    // need?
    if need.is_some() {
        query = query.filter(expenses::need.eq(need.unwrap()));
    }

    query.first(&conn).unwrap()
}
