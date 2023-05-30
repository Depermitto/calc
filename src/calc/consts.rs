use std::collections::HashMap;
use lazy_static::lazy_static;

use super::symbol::{Op, OpType::*};

lazy_static! {
    pub static ref OPS: HashMap<&'static str, Op> = {
        HashMap::from_iter([
            ("(", Op::from_some(LParenthesis, 0)),
            ("-", Op::from_some(Arithmetic, 1)),
            ("+", Op::from_some(Arithmetic, 1)),
            (")", Op::from_some(RParenthesis, 1)),
            ("*", Op::from_some(Arithmetic, 2)),
            ("/", Op::from_some(Arithmetic, 2)),
            ("%", Op::from_some(Arithmetic, 2)),
            ("^", Op::from_some(Arithmetic, 3)),
            ("!", Op::from_some(Arithmetic, 3)),
        ])
    };
}