mod error;
mod calculator {
    pub mod calc;
    pub mod expression;
}

use calculator::calc::Calc;

fn main() {
    let mut c = Calc::new();
    match c.expression.push("") {
        Ok(()) => println!("{}", c.expression.get()),
        Err(e) => println!("{}", e)
    }
}