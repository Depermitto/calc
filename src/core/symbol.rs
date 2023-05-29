use std::num::ParseFloatError;
use crate::consts::OP_WEIGHTS;

#[derive(Debug, Clone)]
pub enum Symbol {
    Digit(f64),
    // Value, weight, // associativity
    Operator(String, i32),

    Func(String),

    Unrecognized(String)
}

// #[derive(Debug, Clone, PartialEq, Copy)]
// pub enum Associativity {
//     Left,
//     Right,
//     Non,
// }

impl Symbol {
    pub fn is_operator(value: &str) -> Option<i32> {
        for (key, weight) in OP_WEIGHTS.clone().into_iter() {
            if key.contains(value) {
                return Some(weight);
            }
        }
        None
    }

    // pub fn associativity(value: &str) -> Associativity {
    //     for (key, assioc) in OP_ASSIOC.clone().into_iter() {
    //         if key.contains(value) {
    //             return assioc
    //         }
    //     }
    //     Associativity::Non
    // }

    pub fn is_digit(value: &str) -> Result<f64, ParseFloatError> {
        value.parse::<f64>()
    }

    pub fn get(&self) -> String {
        match self {
            Self::Digit(d) => d.to_string(),
            Self::Operator(o, _) => o.to_string(),
            _ => "".to_string()
        }
    }
}

impl<T: Into<String>> From<T> for Symbol {
    fn from(value: T) -> Self {
        let value: String = value.into();
        if let Ok(d) = Symbol::is_digit(&value) {
            Symbol::Digit(d)
        } else if let Some(w) = Symbol::is_operator(&value) {
            Symbol::Operator(value, w)
        } else {
            Symbol::Unrecognized(value)
        }
    }
}