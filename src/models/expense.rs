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
    pub need: bool,
    pub created_at: NaiveDateTime,
}

/// used for inserting new expense
#[derive(Debug, Deserialize, Insertable)]
#[table_name = "expenses"]
pub struct NewExpense {
    pub user_id: i32,
    pub amount: f32,
    pub expense_type: String,
    pub need: bool,
}

/// Expense types in string
pub enum ExpenseType {
    Food,
    Transportation,
    Rent,
    Utilities,
    Loan,
    Savings,
}

impl ExpenseType {
    fn value(&self) -> String {
        let res = match self {
            ExpenseType::Food => "food",
            ExpenseType::Transportation => "transportation",
            ExpenseType::Rent => "rent",
            ExpenseType::Utilities => "utilities",
            ExpenseType::Loan => "loan",
            ExpenseType::Savings => "savings",
        };

        String::from(res)
    }

    fn from(type_str: &str) -> Option<ExpenseType> {
        match type_str {
            "food" => Some(ExpenseType::Food),
            "transportation" => Some(ExpenseType::Transportation),
            "rent" => Some(ExpenseType::Rent),
            "utilities" => Some(ExpenseType::Utilities),
            "loan" => Some(ExpenseType::Loan),
            "savings" => Some(ExpenseType::Savings),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! value_cmp {
        ($str:expr, $exp_type:expr) => {
            // $str
            let test = $exp_type;
            assert_eq!($str, test.value());
        };
    }

    #[test]
    fn test_expense_type_value() {
        value_cmp!("food", ExpenseType::Food);
        value_cmp!("transportation", ExpenseType::Transportation);
        value_cmp!("rent", ExpenseType::Rent);
        value_cmp!("utilities", ExpenseType::Utilities);
        value_cmp!("loan", ExpenseType::Loan);
        value_cmp!("savings", ExpenseType::Savings);
    }

    #[test]
    #[should_panic]
    fn test_expense_type_value_should_fail() {
        value_cmp!("food!", ExpenseType::Food);
    }
}
