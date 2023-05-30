use std::ops::{Add, AddAssign};
use super::consts::OPS;

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
    pub fn new() -> Self {
        Self { value: String::new(), kind: SymbolKind::new() }
    }

    pub fn auto(value: &str) -> Self {
        Self { value: value.to_string(), kind: value.to_state() }
    }

    pub fn with_state(value: &str, kind: SymbolKind) -> Self {
        Self { value: value.to_string(), kind: kind }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn kind(&self) -> SymbolKind {
        self.kind
    }

    pub fn clear(&mut self) {
        self.value.clear();
        self.kind.clear();
    }

    pub fn empty(&self) -> bool {
        self.value.is_empty()
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SymbolKind {
    Digit(f64),

    Var,

    Func,

    Dot,

    LeftParths,

    RightParths,
    /// Represents arithmetic operators, each one of
    /// them hold a certain weight.
    Op(i32),

    None,
}

impl SymbolKind {
    pub fn new() -> Self {
        SymbolKind::None
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }
}


pub trait ToState {
    fn to_state(self) -> SymbolKind;
}

pub trait ToSymbol {
    fn to_symbol(self) -> Symbol;
}

impl<T> ToState for T
where
    T: ToString,
    String: From<T>
{
    fn to_state(self) -> SymbolKind {
        let s: String = self.into();
        if let Ok(num) = s.parse::<f64>() {
            SymbolKind::Digit(num)
        } else if let Some(w) = OPS.get(s.as_str()) {
            SymbolKind::Op(w.clone())
        } else if s == "." {
            SymbolKind::Dot
        } else if s == "(" {
            SymbolKind::LeftParths
        } else if s == ")" {
            SymbolKind::RightParths
        } else {
            SymbolKind::None
        }
    }
}

impl<T> ToSymbol for T
where
    T: ToState,
    String: From<T>
{
    fn to_symbol(self) -> Symbol {
        let s: String = self.into();
        Symbol::auto(&s)
    }
}

/// Can be used for adding Digit Symbols
impl Add for Symbol {
    type Output = Symbol;

    fn add(self, rhs: Self) -> Self::Output {
        let value = self.value + rhs.value();
        Symbol::with_state(&value, value.clone().to_state())
    }
}

impl AddAssign for Symbol {
    fn add_assign(&mut self, rhs: Self) {
        let symbol = self.clone() + rhs;
        self.value = symbol.value;
        self.kind = symbol.kind;
    }
}