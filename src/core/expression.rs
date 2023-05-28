use crate::error::CalcError;

/// Used strictly for mathematical expressions like
/// **(8 + 7) * 4**. Accepts any amount of whitespace
pub struct Expression<'a> {
    value: String,
    supported: Vec<&'a str>,
}

impl<'a> Expression<'a> {
    /// Creates a new empty `Expression`
    pub fn new() -> Self {
        Self {
            value: String::new(),
            // Obviously every number is supported, these are only
            // operators and special functions
            supported: vec![
                "+", "-", "*", "/", "%", "^", "!", "//", "(", ")", "min", "max"
            ]
        }
    }

    /// Converts `value` from infix _(normal)_ notation to
    /// **Reverse Polish Notation** using the dijkstra algorithm
    pub fn dijkstrify(&mut self) -> Result<&str, CalcError> {
        todo!()
    }

    /// Consumes `Self::value` inside and returns a `Result`. Assumes
    /// `Self::value` is in **Reverse Polish Notation**, otherwise or in case
    /// of failure throws `CalcError`
    pub fn calc(&mut self) -> Result<f64, CalcError> {
        todo!()
    }

    /// Appends `Self::value` by `value` which is heavily constrained
    /// for mathematical purposes
    pub fn push(&mut self, value: &str) -> Result<(), CalcError> {
        // Trim whitespaces
        let trimmed = value.trim().replace(" ", "");
        let mut copy = trimmed.clone();
        // Remove any numbers and characters inside `self.supported`
        copy.retain(|c| !char::is_numeric(c));
        copy.retain(|c| !self.supported.contains(&c.to_string().as_str()));

        return match copy.is_empty() {
            true => {
                self.value += &trimmed;
                Ok(())
            }
            // Return Err() with first incorrect character
            false => Err(CalcError::UnsupportedValue(copy.chars().nth(0).unwrap().to_string()))
        };
    }

    /// Sets `Self::value` to `value`
    pub fn set(&mut self, value: &str) -> () {
        self.clear();
        self.push(value);
    }

    /// Is `Self::value` empty?
    pub fn empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Clear `Self::value`
    pub fn clear(&mut self) -> () {
        self.value.clear();
    }

    // Getter for `Self::value`
    pub fn get(&self) -> &str {
        &self.value
    }
}