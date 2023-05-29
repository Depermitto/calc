mod error;
mod core {
    pub mod calc;
    pub mod expression;
    pub mod symbol;
}

use crate::core::calc::Calc;
use crate::core::expression::Expression;
use crate::core::symbol::Symbol;

fn main() {
    // let mut c = Calc::new();
    // match c.evaluate("(1 + 7) * 8") {
    //     Ok(num) => println!("{}", num),
    //     Err(e) => println!("{}", e)
    // }
    let a: Symbol = Symbol::from("890.9");
    let b: Symbol = "!".into();
    println!("{:?}\n{:?}", a, b);

    let mut e = Expression::new();
    e.push("10.4 + (7 * 4)");
    println!("{:?}", e)
}