use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref OP_WEIGHTS: HashMap<&'static str, i32> = {
        HashMap::from_iter([
            ("(", 0),
            ("+-)", 1),
            ("*/%", 2),
            ("^!", 3),
        ])
    };

    // pub static ref OP_ASSIOC: HashMap<&'static str, Associativity> = {
    //     HashMap::from_iter([
    //         ("+-*/%!", Left),
    //         ("^", Right),
    //     ])
    // };
}