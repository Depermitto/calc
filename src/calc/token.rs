use super::error::CalcError;

/// Represents a string literal with one of 4 States
/// - Digit - holds information about a number
/// - Op - represents an arithmetic operator with a specified kind
/// - LeftParths - (
/// - RightParths - )
#[derive(Debug, Clone, Copy)]
pub enum Token {
    Digit(f64),

    LeftParths,

    RightParths,
    /// Reresents arithmetic operators, each one of
    Op(Op),
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Plus,
    Minus,
    Factorial,
    Multiply,
    Division,
    Power,
    ModuloDivision,
}

impl Token {
    pub fn new(value: &str) -> Result<Self, CalcError> {
        if let Ok(d) = value.parse::<f64>() {
            return Ok(Self::Digit(d));
        }
        match value {
            "(" => Ok(Self::LeftParths),
            ")" => Ok(Self::RightParths),
            "+" => Ok(Self::Op(Op::Plus)),
            "-" => Ok(Self::Op(Op::Minus)),
            "!" => Ok(Self::Op(Op::Factorial)),
            "%" => Ok(Self::Op(Op::ModuloDivision)),
            "*" => Ok(Self::Op(Op::Multiply)),
            "/" => Ok(Self::Op(Op::Division)),
            "^" => Ok(Self::Op(Op::Power)),
            _ => Err(CalcError::UnsupportedValue(value.to_string())),
        }
    }

    pub fn as_str(&self) -> String {
        match self {
            Token::Digit(d) => d.to_string(),
            Token::LeftParths => "(".to_string(),
            Token::RightParths => ")".to_string(),
            Token::Op(op) => op.as_str().to_string(),
        }
    }
}

impl Op {
    pub fn as_str(&self) -> &str {
        match self {
            Op::Plus => "+",
            Op::Minus => "-",
            Op::Factorial => "!",
            Op::Multiply => "*",
            Op::Division => "/",
            Op::Power => "^",
            Op::ModuloDivision => "%",
        }
    }

    pub fn weight(&self) -> i32 {
        match self {
            Op::Plus => 1,
            Op::Minus => 1,
            Op::Factorial => 3,
            Op::Multiply => 2,
            Op::Division => 2,
            Op::Power => 3,
            Op::ModuloDivision => 2,
        }
    }

    pub fn is_unary(&self) -> bool {
        match self {
            Op::Plus | Op::Minus | Op::Factorial => true,
            _ => false,
        }
    }

    pub fn is_binary(&self) -> bool {
        match self {
            Op::Plus | Op::Minus => true,
            _ => !self.is_unary(),
        }
    }

    fn fractal(number: f64) -> f64 {
        let number = number as i128;
        let mut result: i128 = 1;
        for step in (1..=number).rev() {
            result *= step;
        }
        result as f64
    }

    /// Call the operator on a single argument
    pub fn call_single(&self, number: f64) -> Result<f64, CalcError> {
        match self {
            Op::Plus => Ok(number),
            Op::Minus => Ok(0.0 - number),
            Op::Factorial => Ok(Self::fractal(number)),
            _ => Err(CalcError::IncorrectNumberOfArgs),
        }
    }

    /// Call the operator
    pub fn call_double(&self, first: f64, second: f64) -> Result<f64, CalcError> {
        match self {
            Op::Plus => Ok(first + second),
            Op::Factorial => Err(CalcError::IncorrectNumberOfArgs),
            Op::Minus => Ok(first - second),
            Op::Multiply => Ok(first * second),
            Op::Division => {
                if second == 0.0 {
                    Err(CalcError::ZeroDivision)
                } else {
                    Ok(first / second)
                }
            }
            Op::Power => Ok(first.powf(second)),
            Op::ModuloDivision => Ok(first % second),
        }
    }
}

pub trait ToToken {
    fn try_to_token(self) -> Result<Token, CalcError>;
}

impl<T> ToToken for T
where
    T: ToString,
    String: From<T>,
{
    fn try_to_token(self) -> Result<Token, CalcError> {
        let s: String = self.into();
        Token::new(&s)
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl TryFrom<&str> for Token {
    type Error = CalcError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
