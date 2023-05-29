use std::num::ParseFloatError;

#[derive(Debug, Clone)]
pub enum Symbol {
    Digit(f64),
    Operator(String),
    Func(String),
    Unrecognized(String)
}

impl Symbol {
    pub fn is_operator(value: &str) -> bool {
        vec!["+", "-", "*", "/", "%", "^", "!", "(", ")", "."].contains(&value)
    }

    pub fn is_digit(value: &str) -> Result<f64, ParseFloatError> {
        value.parse::<f64>()
    }
}

impl<T: Into<String>> From<T> for Symbol {
    fn from(value: T) -> Self {
        let value: String = value.into();
        if let Ok(d) = Symbol::is_digit(&value) {
            Symbol::Digit(d)
        } else if Symbol::is_operator(&value) {
            Symbol::Operator(value)
        } else {
            Symbol::Unrecognized(value)
        }
    }
}