use crate::error::CalcError;
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
    pub fn dijkstrify(&mut self) -> Result<&str, CalcError> {
        todo!()
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
                Operator(o) => self.value.push(o.into()),
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
}