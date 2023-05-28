use crate::error::CalcError;
use super::expression::Expression;

pub struct Calc<'a> {
    pub expression: Expression<'a>,
    result: Result<f64, CalcError>,
}

impl<'a> Calc<'a> {
    /// Creates a new `Calc` with `expression` empty and `result` equal to 0.0
    pub fn new() -> Self {
        Self { expression: Expression::new(), result: Result::Ok(0.0) }
    }
}