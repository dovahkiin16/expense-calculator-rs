mod expenses_errors;
mod users_errors;

pub use expenses_errors::*;
pub use users_errors::*;

use serde::Serialize;

/// used for sending error messages as a serializable
#[derive(Debug, Serialize)]
struct ErrorMessage {
    message: String,
}
