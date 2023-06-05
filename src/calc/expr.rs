use super::error::CalcError;
use super::token::{Token::{self, *}, ToToken};

/// macro thanks to [clap](https://github.com/clap-rs/clap/blob/e70b289c5ed43c841bdc11b712c1b2931036c6cd/src/macros.rs#L192-L206)
macro_rules! for_match {
    ($it:ident, $($p:pat => $($e:expr);+),*) => {
        for i in $it.clone() {
            match i {
            $(
                $p => { $($e)+ }
            )*
            }
        }
    };
}

/// Used strictly for mathematical expressions like
/// **(8 + 7) * 4**. Accepts any amount of whitespace
#[derive(Debug, Clone)]
pub struct Expr {
    tokens: Vec<Token>,
}

impl Expr {
    /// Creates a new empty `Expression`
    pub fn new() -> Self {
        Self { tokens: vec![] }
    }

    /// Converts `value` from infix _(normal)_ notation to
    /// **Reverse Polish Notation** using the dijkstra algorithm
    ///
    /// Will catch badly placed parenthesis
    fn infix_to_postfix(tokens: Vec<Token>) -> Result<Vec<Token>, CalcError> {
        let mut output: Vec<Token> = vec![];
        let mut operator_stack: Vec<Token> = vec![];
        for_match!(tokens,
            Digit(d) => output.push(Digit(d)),
            LeftParths => operator_stack.push(LeftParths),
            RightParths => loop {
                match operator_stack.last() {
                    Some(LeftParths) => {
                        operator_stack.pop();
                        break;
                    },
                    None => return Err(CalcError::BadParenthesis),
                    _ => output.push(operator_stack.pop().unwrap())
                }
            },
            Op(o) => {
                while let Some(Op(o2)) = operator_stack.last() {
                    if o2.weight() >= o.weight() {
                        output.push(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operator_stack.push(Op(o));
            }
        );
        operator_stack.reverse();
        output.append(&mut operator_stack);

        Ok(output)
    }

    /// Consumes `Self::value` inside and returns a `Result`. Assumes
    /// `Self::value` is in **Reverse Polish Notation**, otherwise or in case
    /// of failure throws `CalcError`
    pub fn calc(&mut self) -> Result<f64, CalcError> {
        let mut stack = Vec::<Token>::new();
        let tokens = self.tokens.clone();
        for_match!(tokens,
            Digit(d) => stack.push(Digit(d)),
            Op(o) => {
                if let Some(Digit(first)) = stack.pop() {
                    let result = match stack.last() {
                        Some(Digit(second)) if o.is_binary() => {
                            let n = o.call_double(*second, first)?;
                            stack.pop();
                            n
                        }
                        _ if o.is_unary() => o.call_single(first)?,
                        _ => return Err(CalcError::BadExpression)
                    };
                    // Push back to the stack
                    stack.push(result.to_string().try_to_token()?);
                } else {
                    return Err(CalcError::BadExpression);
                }
            },
            _ => return Err(CalcError::BadExpression)
        );
        let Some(Digit(result)) = stack.pop() else { return Err(CalcError::BadExpression) };
        Ok(result)
    }

    /// Sets the expression, accepts infix notation and returns postfix
    pub fn set(&mut self, text: &str) -> Result<(), CalcError> {
        self.tokens.clear();
        let text: String = text
            .trim()
            .replace(" ", "")
            .chars()
            .map(|c|
                if c.is_alphanumeric() || c == '.' {
                    c.to_string()
                } else {
                    format!(" {} ", c)
                }
            )
            .collect();
        let text: Vec<Result<Token, CalcError>> = text
            .trim()
            .split(" ")
            .map(|c| c.try_to_token())
            .collect();

        let mut tokens: Vec<Token> = text.into_iter().flatten().collect();
        tokens = Self::infix_to_postfix(tokens)?;

        self.tokens = tokens;
        Ok(())
    }

    /// Clear `Self::value`
    pub fn clear(&mut self) {
        self.tokens.clear();
    }

    pub fn to_str(&self) -> String {
        self
            .tokens
            .iter()
            .map(|o| o.as_str().to_owned() + " ")
            .collect::<String>()
            .trim()
            .to_string()
    }
}