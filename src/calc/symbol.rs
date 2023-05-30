use std::ops::{Add, AddAssign};
use super::consts::OPS;

/// Represents a string literal with one of 4 States
/// - Digit - holds information about a number
/// - Op - represents an arithmetic operator with a specified type
/// - Func - represents a function
/// - LeftParths - (
/// - RightParths - )
/// - Dot - .
/// - Var - variables
/// - None - For everything else
#[derive(Debug, Clone)]
pub struct Symbol {
    value: String,
    state: State
}

impl Symbol {
    pub fn new() -> Self {
        Self { value: String::new(), state: State::new() }
    }

    pub fn auto(value: &str) -> Self {
        Self { value: value.to_string(), state: value.to_state() }
    }

    pub fn with_state(value: &str, state: State) -> Self {
        Self { value: value.to_string(), state: state }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn clear(&mut self) {
        self.value.clear();
        self.state.clear();
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
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

impl State {
    pub fn new() -> Self {
        State::None
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }
}


pub trait ToState {
    fn to_state(self) -> State;
}

pub trait ToSymbol {
    fn to_symbol(self) -> Symbol;
}

impl<T> ToState for T
where
    T: ToString,
    String: From<T>
{
    fn to_state(self) -> State {
        let s: String = self.into();
        if let Ok(num) = s.parse::<f64>() {
            State::Digit(num)
        } else if let Some(w) = OPS.get(s.as_str()) {
            State::Op(w.clone())
        } else if s == "." {
            State::Dot
        } else if s == "(" {
            State::LeftParths
        } else if s == ")" {
            State::RightParths
        } else {
            State::None
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
        self.state = symbol.state;
    }
}