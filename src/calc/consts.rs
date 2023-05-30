use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref OPS: HashMap<&'static str, i32> = {
        HashMap::from_iter([
            // ("(", 0),
            ("-", 1),
            ("+", 1),
            // (")", 1),
            ("*", 2),
            ("/", 2),
            ("%", 2),
            ("^", 3),
            ("!", 3),
        ])
    };
}