use std::ops::{Add, AddAssign};
use super::{consts::OPS, error::CalcError};

/// Represents a string literal with one of 4 States
/// - Digit - holds information about a number
/// - Op - represents an arithmetic operator with a specified kind
/// - Func - represents a function
/// - LeftParths - (
/// - RightParths - )
/// - Dot - .
/// - Var - variables
/// - None - For everything else
#[derive(Debug, Clone)]
pub struct Symbol {
    value: String,
    token: Token
}

impl Symbol {
    pub fn new(value: String) -> Result<Self, CalcError> {
        Ok( Self { value: value.clone(), token: value.try_to_token()? } )
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    /// Change the stored value and automatically update the token
    pub fn push_str(&mut self, str: &str) -> Result<(), CalcError> {
        self.value.push_str(str);
        self.token = self.value.clone().try_to_token()?;
        Ok(())
    }

    pub fn token(&self) -> Token {
        self.token
    }

    pub fn clear(&mut self) {
        self.value.clear();
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Digit(f64),

    Dot,

    LeftParths,

    RightParths,
    /// Represents arithmetic operators, each one of
    /// them hold a certain weight.
    Op(i32)
}

impl Token {
    pub fn is_digit(self) -> bool {
        if let Self::Digit(_) = self { true } else { false }
    }
}

pub trait ToToken {
    fn try_to_token(self) -> Result<Token, CalcError>;
}

pub trait ToSymbol {
    fn try_to_symbol(self) -> Result<Symbol, CalcError>;
}

impl<T> ToToken for T
where
    T: ToString,
    String: From<T>
{
    fn try_to_token(self) -> Result<Token, CalcError> {
        let s: String = self.into();
        if let Ok(num) = s.parse::<f64>() {
            return Ok(Token::Digit(num))
        } else if let Some(w) = OPS.get(s.as_str()) {
            return Ok(Token::Op(w.clone()))
        }
        match s.as_str() {
            "." => Ok(Token::Dot),
            "(" => Ok(Token::LeftParths),
            ")" => Ok(Token::RightParths),
            _ => Err(CalcError::UnsupportedValue(s))
        }
    }
}

impl<T> ToSymbol for T
where
    T: ToToken,
    String: From<T>
{
    fn try_to_symbol(self) -> Result<Symbol, CalcError> {
        let s: String = self.into();
        Symbol::new(s)
    }
}

/// Can be used for adding Digit Symbols
impl Add for Symbol {
    type Output = Symbol;

    fn add(self, rhs: Self) -> Self::Output {
        let value = self.value + rhs.value();
        Symbol::new(value).unwrap()
    }
}

impl AddAssign for Symbol {
    fn add_assign(&mut self, rhs: Self) {
        let symbol = self.clone() + rhs;
        self.value = symbol.value;
        self.token = symbol.token;
    }
}