mod calc {
    pub mod calc;
    pub mod consts;
    pub mod error;
    pub mod expression;
    pub mod symbol;
}

use calc::{symbol::Symbol, symbol::*};
use calc::expression::Expression;

fn main() {
    // let mut c = Calc::new();
    // match c.evaluate("(1 + 7) * 8") {
    //     Ok(num) => println!("{}", num),
    //     Err(e) => println!("{}", e)
    // }
    //
    let mut t = Expression::new();
    t.push("(1%^!!8)*(1+(12+4%8!))");
    println!("{:#?}", t.to_str());
    t.dijkstrify();
    println!("{:#?}", t.to_str());
}