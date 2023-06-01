use super::error::CalcError;
use super::expr::Expr;

pub struct Calc {
    expression: Expr,
    result: Option<f64>,
}

impl Calc {
    /// Creates a new `Calc` with `expression` empty and `result` equal to 0.0
    pub fn new() -> Self {
        Self { expression: Expr::new(), result: None }
    }

    pub fn result(&self) -> Option<f64> {
        self.result.clone()
    }

    pub fn evaluate(&mut self, value: &str) -> Result<f64, CalcError> {
        self.expression.set(value)?;
        self.expression.dijkstrify()?;

        let outcome = self.expression.calc();
        match outcome {
            Ok(out) => self.result = Some(out),
            Err(e) => return Err(e)
        }
        Ok(self.result().unwrap())
    }
}