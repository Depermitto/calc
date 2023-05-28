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

    /// Creates a new `Expression` with **Reverse Polish Notation**
    /// value using the dijkstra algorithm
    pub fn dijkstra(input: &str) -> Self {
        todo!()
    }

    /// Consumes `value` inside and returns a `Result`. Assumes
    /// `value` is in **Reverse Polish Notation**, otherwise or in case
    /// of failure throws `CalcError`
    pub fn rpn_calc(&mut self) -> Result<f64, CalcError> {
        todo!()
    }

    /// Appends `Self::value` by `value` which is heavily constrained
    /// for mathematical purposes
    pub fn push(&mut self, value: &str) -> Result<(), CalcError> {
        // Trim whitespaces
        let mut trimmed = value.trim().replace(" ", "");
        // Remove any numbers and characters inside `self.supported`
        trimmed.retain(|c| !char::is_numeric(c));
        trimmed.retain(|c| !self.supported.contains(&c.to_string().as_str()));

        if let false = trimmed.is_empty() {
            return Err(CalcError::UnsupportedValue(trimmed));
        } else {
            self.value += value;
            return Ok(())
        }
    }

    /// Sets `Self::value` to `value`
    pub fn edit(&mut self, value: &str) -> () {
        self.value = String::from(value);
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