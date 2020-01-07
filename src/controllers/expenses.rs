use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

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

pub async fn find_all(info: web::Path<PathUserId>) -> impl Responder {
    let user_expenses = expense_db::find_all(info.user_id);
    let body = serde_json::to_string(&user_expenses).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
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

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[derive(Debug, Serialize)]
pub struct TotalExpense {
    total_expense: f32
}

pub async fn get_total_expense(info: web::Path<PathUserId>) -> impl Responder {
    let expense_sum = expense_db::get_expenses_sum(info.user_id).unwrap();
    let body_struct = TotalExpense {
        total_expense: expense_sum
    };

    let body = serde_json::to_string(&body_struct).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}
