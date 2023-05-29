mod calc {
    pub mod calc;
    pub mod consts;
    pub mod error;
    pub mod expression;
    pub mod symbol;
}

use calc::expression::Expression;

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