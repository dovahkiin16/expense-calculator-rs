extern crate openssl;
#[macro_use]
extern crate diesel;
extern crate expense_calculator;

fn main() {
    let _ = expense_calculator::run().unwrap();
}
