use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Expense {
    pub id: i32,
    pub amount: f32,
    pub expense_type: String,
    pub created_at: NaiveDateTime,
}
