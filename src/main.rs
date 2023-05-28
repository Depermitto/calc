mod error;
mod core {
    pub mod calc;
    pub mod expression;
}

use crate::core::calc::Calc;

fn main() {
    let mut c = Calc::new();
    match c.evaluate("(1 + 7) * 8") {
        Ok(num) => println!("{}", num),
        Err(e) => println!("{}", e)
    }
}