mod calc {
    pub mod calc;
    pub mod error;
    pub mod expr;
    pub mod symbol;
}

use calc::calc::Calc;

fn main() {
    let mut c = Calc::new();
    match c.evaluate("40 % 8") {
        Ok(num) => println!("{}", num),
        Err(e) => println!("{}", e)
    }
}