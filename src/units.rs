use std::fmt;

#[derive(PartialEq, PartialOrd)]
pub struct Kilometers(pub f64);

impl fmt::Display for Kilometers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.*} Km", 3, self.0)
    }
}
