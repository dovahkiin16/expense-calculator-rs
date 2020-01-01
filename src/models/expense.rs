use chrono::NaiveDateTime;
use crate::models::User;

#[derive(Debug, Identifiable, Associations, Queryable)]
#[belongs_to(User)]
pub struct Expense {
    pub id: i32,
    pub user_id: i32,
    pub amount: f32,
    pub expense_type: String,
    pub created_at: NaiveDateTime,
}