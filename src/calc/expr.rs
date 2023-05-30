use super::error::CalcError;
use super::symbol::{Symbol, SymbolKind::*, ToSymbol, self};

/// Used strictly for mathematical expressions like
/// **(8 + 7) * 4**. Accepts any amount of whitespace
#[derive(Debug, Clone)]
pub struct Expr {
    value: Vec<Symbol>,
}

impl Expr {
    /// Creates a new empty `Expression`
    pub fn new() -> Self {
        Self { value: vec![] }
    }

    /// Converts `value` from infix _(normal)_ notation to
    /// **Reverse Polish Notation** using the dijkstra algorithm
    pub fn dijkstrify(&mut self) -> Result<(), CalcError> {
        let mut output: Vec<Symbol> = vec![];
        let mut stack: Vec<Symbol> = vec![];
        'main: for symbol in self.value.iter() {
            let symbol = symbol.clone();
            match symbol.kind() {
                Digit(_) => output.push(symbol),
                LeftParths => stack.push(symbol),
                RightParths => 'till_lparths: while let Some(s) = stack.last() {
                    match s.kind() {
                        LeftParths => { stack.pop(); break 'till_lparths; },
                        _ if stack.len() == 1 => return Err(CalcError::BadParenthesis),
                        _ => output.push(stack.pop().unwrap())
                    };
                },
                Op(weight) => {
                    'priority_queue: while let Some(s) = stack.last() {
                        match s.kind() {
                            Op(w2) if w2 >= weight => output.push(stack.pop().unwrap()),
                            _ => break 'priority_queue
                        }
                    }
                    stack.push(symbol);
                },
                _ => ()
            };
        }
        // Add stack elements to output in reverse order
        stack.reverse();
        output.append(&mut stack);
        // Remove any parenthesis - shouldn't be like this TODO
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
    pub fn push(&mut self, value: &str) {
        // Trim whitespaces
        let trimmed = value.trim().replace(" ", "");
        let mut num = Symbol::new();
        for ch in trimmed.chars() {
            let sym = ch.to_symbol();
            match sym.kind() {
                Dot | Digit(_) => num += sym,
                Op(_) | LeftParths | RightParths => {
                    if !num.empty() {
                        self.value.push(num.clone());
                        num.clear();
                    }
                    self.value.push(sym);
                },
                _ => ()
            }
        }
        self.value.push(num.clone());
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
            .map(|o| o.value().to_owned() + " ")
            .collect::<String>()
            .trim()
            .to_string()
    }
}