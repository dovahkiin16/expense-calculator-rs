use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::models::User;
use crate::schema::expenses;

#[derive(Debug, Identifiable, Associations, Serialize, Queryable)]
#[belongs_to(User)]
pub struct Expense {
    pub id: i32,
    pub user_id: i32,
    pub amount: f32,
    pub expense_type: String,
    pub created_at: NaiveDateTime,
}

/// used for inserting new expense
#[derive(Debug, Deserialize, Insertable)]
#[table_name = "expenses"]
pub struct NewExpense {
    pub user_id: i32,
    pub amount: f32,
    pub expense_type: String,
}
