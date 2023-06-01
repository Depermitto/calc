use super::error::CalcError;

/// Represents a string literal with one of 4 States
/// - Digit - holds information about a number
/// - Op - represents an arithmetic operator with a specified kind
/// - Func - represents a function
/// - LeftParths - (
/// - RightParths - )
/// - Dot - .
/// - Var - variables
/// - None - For everything else
#[derive(Debug, Clone, Copy)]
pub enum Symbol {
    Digit(f64),

    Dot,

    LeftParths,

    RightParths,
    /// Reresents arithmetic operators, each one of
    Op(Op),
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Plus,
    Minus,
    Fractal,
    Multiply,
    Divide,
    Power,
    Remainer,
}

impl Symbol {
    pub fn new(value: &str) -> Result<Self, CalcError> {
        if let Ok(d) = value.parse::<f64>() {
            return Ok(Self::Digit(d));
        }
        match value {
            "." => Ok(Self::Dot),
            "(" => Ok(Self::LeftParths),
            ")" => Ok(Self::RightParths),
            "+" => Ok(Self::Op(Op::Plus)),
            "-" => Ok(Self::Op(Op::Minus)),
            "!" => Ok(Self::Op(Op::Fractal)),
            "%" => Ok(Self::Op(Op::Remainer)),
            "*" => Ok(Self::Op(Op::Multiply)),
            "/" => Ok(Self::Op(Op::Divide)),
            "^" => Ok(Self::Op(Op::Power)),
            _ => Err(CalcError::UnsupportedValue(value.to_string())),
        }
    }

    pub fn as_str(&self) -> String {
        match self {
            Symbol::Digit(d) => d.to_string(),
            Symbol::Dot => ".".to_string(),
            Symbol::LeftParths => "(".to_string(),
            Symbol::RightParths => ")".to_string(),
            Symbol::Op(op) => op.as_str().to_string(),
        }
    }
}


impl Op {
    pub fn as_str(&self) -> &str {
        match self {
            Op::Plus => "+",
            Op::Minus => "-",
            Op::Fractal => "!",
            Op::Multiply => "*",
            Op::Divide => "/",
            Op::Power => "^",
            Op::Remainer => "%",
        }
    }

    pub fn weight(&self) -> i32 {
        match self {
            Op::Plus => 1,
            Op::Minus => 1,
            Op::Fractal => 3,
            Op::Multiply => 2,
            Op::Divide => 2,
            Op::Power => 3,
            Op::Remainer => 2,
        }
    }

    pub fn call(&self, first: f64, second: f64) -> Result<f64, CalcError> {
        match self {
            Op::Plus => Ok(first + second),
            Op::Fractal => Err(CalcError::UnsupportedValue(String::from("!"))),
            Op::Minus => Ok(first - second),
            Op::Multiply => Ok(first * second),
            Op::Divide => {
                if second == 0.0 {
                    Err(CalcError::ZeroDivision)
                } else {
                    Ok(first / second)
                }
            }
            Op::Power => Ok(first * second),
            Op::Remainer => Ok(first % second),
        }
    }
}

pub trait ToSymbol {
    fn try_to_symbol(self) -> Result<Symbol, CalcError>;
}

impl<T> ToSymbol for T
where
    T: ToString,
    String: From<T>,
{
    fn try_to_symbol(self) -> Result<Symbol, CalcError> {
        let s: String = self.into();
        Symbol::new(&s)
    }
}

impl ToString for Symbol {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl TryFrom<&str> for Symbol {
    type Error = CalcError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
