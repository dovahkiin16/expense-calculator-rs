extern crate diesel;
extern crate expense_calculator;
extern crate openssl;

fn main() {
    let _ = expense_calculator::run().unwrap();
}
