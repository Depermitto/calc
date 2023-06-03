mod calc {
    pub mod calc;
    pub mod error;
    pub mod expr;
    pub mod token;
}

use calc::calc::Calc;
use inquire::Text;

fn main() {
    let mut c = Calc::new();
    loop {
        let text = Text::new("Expression: ")
            .with_placeholder("(2 + 1) / 8")
            .prompt();

        match text {
            Ok(string) => match c.evaluate(string.as_str()) {
                Ok(num) => println!("{}", num),
                Err(e) => println!("{}", e)
            }
            Err(_) => { println!("Improper inquiry\nExiting..."); break; }
        }
    }
}