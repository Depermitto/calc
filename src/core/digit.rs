#[derive(Debug, Clone)]
pub enum Digit<'a> {
    Number(f64),
    Operator(&'a str),
    Unrecognized
}

impl<'a> Digit<'a> {
    pub fn operators() -> Vec<&'a str> {
        vec!["+", "-", "*", "/", "%", "^", "!", "(", ")"]
    }
}

impl<'a> Into<Digit<'a>> for &str {
    fn into(self) -> Digit<'a> {
        if let Ok(c) = self.parse::<f64>() {
            Digit::Number(c)
        } else if Digit::operators().contains(&self) {
            Digit::Operator(self)
        } else {
            Digit::Unrecognized
        }
    }
}