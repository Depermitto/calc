use thiserror::Error;

#[derive(Debug, Error)]
pub enum CalcError {
    #[error("Could not finish the calculation properly")]
    CalculationError,

    #[error("Could not read the expression")]
    RpnError,

    #[error("{0} was not recognized")]
    UnsupportedValue(String)
}