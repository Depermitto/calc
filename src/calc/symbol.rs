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
    kind: SymbolKind
}

impl Symbol {
    pub fn new(value: String) -> Result<Self, CalcError> {
        Ok( Self { value: value.clone(), kind: value.try_to_kind()? } )
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn kind(&self) -> SymbolKind {
        self.kind
    }

    pub fn clear(&mut self) {
        self.value.clear();
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SymbolKind {
    Digit(f64),

    Dot,

    LeftParths,

    RightParths,
    /// Represents arithmetic operators, each one of
    /// them hold a certain weight.
    Op(i32)
}

pub trait ToKind {
    fn try_to_kind(self) -> Result<SymbolKind, CalcError>;
}

pub trait ToSymbol {
    fn try_to_symbol(self) -> Result<Symbol, CalcError>;
}

impl<T> ToKind for T
where
    T: ToString,
    String: From<T>
{
    fn try_to_kind(self) -> Result<SymbolKind, CalcError> {
        let s: String = self.into();
        if let Ok(num) = s.parse::<f64>() {
            return Ok(SymbolKind::Digit(num))
        } else if let Some(w) = OPS.get(s.as_str()) {
            return Ok(SymbolKind::Op(w.clone()))
        }
        match s.as_str() {
            "." => Ok(SymbolKind::Dot),
            "(" => Ok(SymbolKind::LeftParths),
            ")" => Ok(SymbolKind::RightParths),
            _ => Err(CalcError::UnsupportedValue(s))
        }
    }
}

impl<T> ToSymbol for T
where
    T: ToKind,
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
        self.kind = symbol.kind;
    }
}