mod calc {
    pub mod calc;
    pub mod consts;
    pub mod error;
    pub mod expr;
    pub mod symbol;
}

use calc::{symbol::Symbol, symbol::*};
use calc::expr::Expr;

fn main() {
    // let mut c = Calc::new();
    // match c.evaluate("(1 + 7) * 8") {
    //     Ok(num) => println!("{}", num),
    //     Err(e) => println!("{}", e)
    // }
    //
    let mut t = Expr::new();
    t.push("3+4*2/(1-5)^2)");
    println!("{:#?}", t.to_str());
    t.dijkstrify();
    println!("{:#?}", t.to_str());
}