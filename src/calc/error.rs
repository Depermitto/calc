use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum CalcError {
    #[error("Could not finish the calculation properly")]
    CalculationError,

    #[error("Cannot divide by 0")]
    ZeroDivision,

    #[error("Could not read the expression")]
    BadExpression,

    #[error("Bad parenthesis")]
    BadParenthesis,

    #[error("{0} was not recognized")]
    UnsupportedValue(String),
}