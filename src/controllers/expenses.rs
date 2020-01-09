use actix_web::{web, Responder};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::controllers::errors::ExpenseError;
use crate::database::expense as expense_db;
use crate::models::NewExpense;

/// used to get user_id in the URL path
#[derive(Deserialize)]
pub struct PathUserId {
    pub user_id: i32,
}

/// expense post request form
#[derive(Deserialize)]
pub struct NewExpenseForm {
    pub amount: f32,
    pub expense_type: String,
    pub need: bool,
}

/// expense result serializer
#[derive(Debug, Serialize)]
pub struct TotalExpense {
    total_expense: f32,
    exp_type: Option<String>,
    from: Option<NaiveDateTime>,
    until: Option<NaiveDateTime>,
}

/// expense input query dto
#[derive(Debug, Deserialize)]
pub struct ExpenseQuery {
    exp_type: Option<String>,
    from: Option<NaiveDateTime>,
    until: Option<NaiveDateTime>,
}

pub async fn find_all(info: web::Path<PathUserId>) -> impl Responder {
    let user_expenses = expense_db::find_all(info.user_id);
    let body = serde_json::to_string(&user_expenses).unwrap();

    super::send_ok(body)
}

pub async fn add_one(
    item: web::Json<NewExpenseForm>,
    info: web::Path<PathUserId>,
) -> impl Responder {
    let insertable_expense = NewExpense {
        user_id: info.user_id,
        expense_type: item.expense_type.clone(),
        amount: item.amount,
        need: item.need,
    };

    let new_expense = expense_db::add_one(insertable_expense).expect("Adding expense error");
    let body = serde_json::to_string(&new_expense).unwrap();

    super::send_ok(body)
}

pub async fn get_total_expense(
    path: web::Path<PathUserId>,
    query: web::Query<ExpenseQuery>,
) -> Result<impl Responder, ExpenseError> {
    let exp_sum =
        expense_db::get_expenses_sum(path.user_id, query.exp_type.clone(), query.from, query.until)
            .unwrap();

    let body_struct = TotalExpense {
        total_expense: exp_sum,
        exp_type: query.exp_type.clone(),
        from: query.from,
        until: query.until,
    };

    let body = serde_json::to_string(&body_struct).unwrap();

    Ok(super::send_ok(body))
}
