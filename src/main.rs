mod error;
mod core {
    pub mod calc;
    pub mod expression;
    pub mod symbol;
}
mod consts;

use crate::core::expression::Expression;

fn main() {
    // let mut c = Calc::new();
    // match c.evaluate("(1 + 7) * 8") {
    //     Ok(num) => println!("{}", num),
    //     Err(e) => println!("{}", e)
    // }
    let mut t = Expression::new();
    t.push("12+8*7");
    t.dijkstrify();
    println!("{:#?}", t.to_str());
}