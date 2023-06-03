use super::error::CalcError;
use super::token::{Token::{self, *}, ToSymbol};

/// Used strictly for mathematical expressions like
/// **(8 + 7) * 4**. Accepts any amount of whitespace
#[derive(Debug, Clone)]
pub struct Expr {
    tokens: Vec<Token>,
}

impl Expr {
    /// Creates a new empty `Expression`
    pub fn new() -> Self {
        Self { tokens: vec![] }
    }

    /// Converts `value` from infix _(normal)_ notation to
    /// **Reverse Polish Notation** using the dijkstra algorithm
    ///
    /// Will catch badly placed parenthesis and dots
    pub fn dijkstrify(&mut self) -> Result<(), CalcError> {
        let mut output: Vec<Token> = vec![];
        let mut op_stack: Vec<Token> = vec![];
        for token in self.tokens.iter() {
            let token = token.clone();
            match token {
                Digit(_) => output.push(token),
                LeftParths => op_stack.push(token),
                RightParths => loop {
                    match op_stack.last() {
                        Some(LeftParths) => { op_stack.pop(); break; }
                        Some(_) if op_stack.len() == 1 => return Err(CalcError::BadParenthesis),
                        Some(_) => output.push(op_stack.pop().unwrap()),
                        None => return Err(CalcError::BadParenthesis),
                    }
                }
                Op(o1) => {
                    while let Some(Op(o2)) = op_stack.last() {
                        if o1.weight() <= o2.weight() {
                            output.push(op_stack.pop().unwrap())
                        } else { break; }
                    }
                    op_stack.push(token);
                }
                // Dot won't appear anyway
                Dot => return Err(CalcError::BadExpression)
            };
        }
        // Add stack elements to output in reverse order
        op_stack.reverse();
        output.append(&mut op_stack);
        // Remove any parenthesis - shouldn't be like this TODO
        self.tokens = output;

        Ok(())
    }

    /// Consumes `Self::value` inside and returns a `Result`. Assumes
    /// `Self::value` is in **Reverse Polish Notation**, otherwise or in case
    /// of failure throws `CalcError`
    pub fn calc(&mut self) -> Result<f64, CalcError> {
        let mut stack: Vec<Token> = vec![];
        for token in &self.tokens {
            match token {
                Digit(_) => stack.push(token.clone()),
                Op(o) => {
                    if let Some(Digit(a)) = stack.pop() {
                        if let Some(Digit(b)) = stack.pop() {
                            let result = o.call(b, a)?;
                            stack.push(result.to_string().try_to_symbol()?);
                        }
                    }
                    else { return Err(CalcError::BadExpression) }
                }
                _ => ()
            }
        }

        if let Some(Digit(d)) = stack.pop() {
            Ok(d)
        } else {
            Err(CalcError::CalculationError)
        }
    }

    /// Appends `Self::value` by `value` which is heavily constrained
    /// for mathematical purposes
    pub fn push(&mut self, value: &str) -> Result<(), CalcError> {
        // Trim whitespaces
        let trimmed = value.trim().replace(" ", "");
        let mut digit_parts = String::new();
        for ch in trimmed.chars() {
            let token = ch.try_to_symbol()?;
            match token {
                Digit(_) | Dot => digit_parts.push(ch),
                _ => {
                    if let Ok(s) = digit_parts.as_str().try_to_symbol() {
                        self.tokens.push(s);
                        digit_parts.clear();
                    }
                    self.tokens.push(token);
                }
            }
        }
        if let Ok(s) = digit_parts.try_to_symbol() {
            self.tokens.push(s);
        }
        Ok(())
    }

    /// Sets `Self::value` to `value`
    pub fn set(&mut self, value: &str) -> Result<(), CalcError> {
        self.clear();
        self.push(value)?;
        Ok(())
    }

    /// Clear `Self::value`
    pub fn clear(&mut self) {
        self.tokens.clear();
    }

    pub fn to_str(&self) -> String {
        self
            .tokens
            .iter()
            .map(|o| o.as_str().to_owned() + " ")
            .collect::<String>()
            .trim()
            .to_string()
    }
}