use std::{num::ParseFloatError, marker::PhantomData};
use super::consts::OP_WEIGHTS;

pub struct Digit(f64);

#[derive(Debug, Clone)]
pub struct Symbol<State = Digit> {
    value: String,
    state: PhantomData<State>
}
