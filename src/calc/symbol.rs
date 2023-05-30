use super::consts::OPS;

#[derive(Debug, Clone)]
pub struct Digit(Option<f64>);

#[derive(Debug, Clone)]
pub struct Var;

#[derive(Debug, Clone)]
pub struct Func;

#[derive(Debug, Clone)]
pub struct Op {
    optype: Option<OpType>,
    weight: Option<i32>
}

#[derive(Debug, Clone, Copy)]
pub enum OpType {
    Arithmetic,
    LParenthesis,
    RParenthesis,
    Special,
}

impl Op {
    /// Creates empty `Op` with \[`None`] values
    pub fn new() -> Self {
        Self { optype: None, weight: None }
    }

    /// Takes the parameters and converts them to \[`Some`]
    pub fn from_some(optype: OpType, weight: i32) -> Self {
        Self { optype: Some(optype), weight: Some(weight) }
    }
}

#[derive(Debug, Clone)]
pub struct Symbol<State = Digit> {
    value: String,
    state: State
}

impl Symbol<Digit> {
    /// Unwrapped version of get_digit
    pub fn digit(&self) -> f64 {
        self.get_digit().unwrap()
    }

    pub fn get_digit(&self) -> Option<f64> {
        self.state.0
    }
}

impl Symbol<Op> {
    /// Unwrapped version of get_optype
    pub fn optype(&self) -> OpType {
        self.get_optype().unwrap()
    }

    pub fn get_optype(&self) -> Option<OpType> {
        self.state.optype
    }

    /// Unwrapped version of get_weight
    pub fn weight(&self) -> i32 {
        self.get_weight().unwrap()
    }

    pub fn get_weight(&self) -> Option<i32> {
        self.state.weight
    }
}

impl<T: Clone> Symbol<T> {
    pub fn new(value: &str, state: T) -> Self {
        Self { value: value.to_string(), state: state }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn state(&self) -> T {
        self.state.clone()
    }
}

impl<T: Into<String>> From<T> for Symbol<Digit> {
    fn from(value: T) -> Self {
        let value: String = value.into();
        match value.parse::<f64>() {
            Ok(num) => Self::new(&value, Digit(Some(num))),
            Err(_) => Self::new(&value, Digit(None))
        }
    }
}

impl<T: Into<String>> From<T> for Symbol<Op> {
    fn from(value: T) -> Self {
        let value: String = value.into();
        match OPS.get(value.as_str()) {
            Some(op) => Self::new(&value, op.clone()),
            None => Self::new(&value, Op::new())
        }
    }
}