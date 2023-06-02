use super::error::CalcError;
use super::symbol::{Symbol::{self, *}, ToSymbol};

/// Used strictly for mathematical expressions like
/// **(8 + 7) * 4**. Accepts any amount of whitespace
#[derive(Debug, Clone)]
pub struct Expr {
    symbols: Vec<Symbol>,
}

impl Expr {
    /// Creates a new empty `Expression`
    pub fn new() -> Self {
        Self { symbols: vec![] }
    }

    /// Converts `value` from infix _(normal)_ notation to
    /// **Reverse Polish Notation** using the dijkstra algorithm
    ///
    /// Will catch badly placed parenthesis and dots
    pub fn dijkstrify(&mut self) -> Result<(), CalcError> {
        let mut output: Vec<Symbol> = vec![];
        let mut op_stack: Vec<Symbol> = vec![];
        for symbol in self.symbols.iter() {
            let symbol = symbol.clone();
            match symbol {
                Digit(_) => output.push(symbol),
                LeftParths => op_stack.push(symbol),
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
                    op_stack.push(symbol);
                }
                // Dot won't appear anyway
                Dot => return Err(CalcError::BadExpression)
            };
        }
        // Add stack elements to output in reverse order
        op_stack.reverse();
        output.append(&mut op_stack);
        // Remove any parenthesis - shouldn't be like this TODO
        self.symbols = output;

        Ok(())
    }

    /// Consumes `Self::value` inside and returns a `Result`. Assumes
    /// `Self::value` is in **Reverse Polish Notation**, otherwise or in case
    /// of failure throws `CalcError`
    pub fn calc(&mut self) -> Result<f64, CalcError> {
        let mut stack: Vec<Symbol> = vec![];
        for sym in &self.symbols {
            match sym {
                Digit(_) => stack.push(sym.clone()),
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
            let sym = ch.try_to_symbol()?;
            if let Digit(_) | Dot = sym {
                digit_parts.push(ch);
                continue;
            }

            if let Ok(s) = digit_parts.as_str().try_to_symbol() {
                self.symbols.push(s);
                digit_parts.clear();
            }

            self.symbols.push(sym);
        }
        if let Ok(s) = digit_parts.try_to_symbol() {
            self.symbols.push(s);
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
        self.symbols.clear();
    }

    pub fn to_str(&self) -> String {
        self
            .symbols
            .iter()
            .map(|o| o.as_str().to_owned() + " ")
            .collect::<String>()
            .trim()
            .to_string()
    }
}