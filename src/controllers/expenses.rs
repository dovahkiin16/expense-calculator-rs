use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::database::expense as expense_db;
use crate::models::{NewExpense, ExpenseType};
use crate::controllers::errors::ExpenseError;

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

/// expense serializer
#[derive(Debug, Serialize)]
pub struct TotalExpense {
    total_expense: f32,
}

/// expense query
#[derive(Debug, Deserialize)]
pub struct ExpenseQuery {
    t: Option<String>, // type
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

    super::send_ok(body)
}

pub async fn get_total_expense(
    path: web::Path<PathUserId>,
    query: web::Query<ExpenseQuery>,
) -> Result<impl Responder, ExpenseError> {
    if query.t.is_some() {
        let exp_type = ExpenseType::from(query.t.as_ref().unwrap());

        if exp_type.is_err() {
            let err = ExpenseError::ValidationError {
                field: String::from("t")
            };
            return Err(err);
        }

        let exp_type = exp_type.unwrap();
        let expense_sum = expense_db::get_expenses_sum_by_type(
            path.user_id,
            &exp_type.value()
        ).unwrap();

        let body_struct = TotalExpense {
            total_expense: expense_sum,
        };
        let body = serde_json::to_string(&body_struct).unwrap();

        Ok(super::send_ok(body))
    } else {
        let expense_sum = expense_db::get_expense_sum_all(path.user_id).unwrap();

        let body_struct = TotalExpense {
            total_expense: expense_sum,
        };
        let body = serde_json::to_string(&body_struct).unwrap();

        Ok(super::send_ok(body))
    }
}
