use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

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
    };

    let new_expense = expense_db::add_one(insertable_expense).expect("Adding expense error");
    let body = serde_json::to_string(&new_expense).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}
