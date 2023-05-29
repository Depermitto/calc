use super::error::CalcError;
use super::symbol::Symbol::{self, *};

/// Used strictly for mathematical expressions like
/// **(8 + 7) * 4**. Accepts any amount of whitespace
#[derive(Debug, Clone)]
pub struct Expression {
    value: Vec<Symbol>,
}

impl Expression {
    /// Creates a new empty `Expression`
    pub fn new() -> Self {
        Self { value: vec![] }
    }

    /// Converts `value` from infix _(normal)_ notation to
    /// **Reverse Polish Notation** using the dijkstra algorithm
    pub fn dijkstrify(&mut self) -> Result<(), CalcError> {
        let mut output: Vec<Symbol> = vec![];
        let mut stack: Vec<Symbol> = vec![];
        for symbol in self.value.iter() {
            match symbol.clone() {
                Digit(d) => output.push(Digit(d)),
                Operator(s, _) if s == "(" => stack.push(symbol.clone()),
                Operator(o1, w1) => {
                    while let Some(Operator(o2, w2)) = stack.last() {
                        if o1 == "(" && o2 == ")" {
                            stack.pop();
                        } else if o1 == "(" && stack.is_empty() {
                            return Err(CalcError::BadParenthesis)
                        } else if (w1 <= *w2) || (o1 == "(" && o2 != ")") {
                            output.push(stack.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    stack.push(symbol.clone());
                }
                _ => return Err(CalcError::BadExpression)
            }
        }
        // Add stack elements to output in reverse order
        stack.reverse();
        output.append(&mut stack);
        // Remove any parenthesis - shouldn't be like this TODO
        output.retain(|o| !"()".contains(&o.get()));
        self.value = output;

        Ok(())
    }

    /// Consumes `Self::value` inside and returns a `Result`. Assumes
    /// `Self::value` is in **Reverse Polish Notation**, otherwise or in case
    /// of failure throws `CalcError`
    pub fn calc(&mut self) -> Result<f64, CalcError> {
        todo!()
    }

    pub fn validate(&self) -> Result<(), CalcError> {
        todo!()
    }

    /// Appends `Self::value` by `value` which is heavily constrained
    /// for mathematical purposes
    pub fn push(&mut self, value: &str) -> () {
        // Trim whitespaces
        let trimmed = value.trim().replace(" ", "");
        for ch in trimmed.chars() {
            let sym = Symbol::from(ch);
            match sym {
                Operator(o, _) => self.value.push(o.into()),
                Digit(d_new) => match self.value.last_mut() {
                    Some(Digit(d_orig)) => *d_orig = *d_orig * 10.0 + d_new,
                    _ => self.value.push(Digit(d_new)),
                }
                _ => self.value.push(Unrecognized(ch.into()))
            };
        }
    }

    /// Sets `Self::value` to `value`
    pub fn set(&mut self, value: &str) -> () {
        self.clear();
        self.push(value);
    }

    /// Is `Self::value` empty?
    pub fn empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Clear `Self::value`
    pub fn clear(&mut self) -> () {
        self.value.clear();
    }

    // Getter for `Self::value`
    pub fn get(&self) -> Vec<Symbol> {
        self.value.clone()
    }

    pub fn to_str(&self) -> String {
        self
            .value
            .iter()
            .map(|o| o.get() + " ")
            .collect::<String>()
            .trim()
            .to_string()
    }
}