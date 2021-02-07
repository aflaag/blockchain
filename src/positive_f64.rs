use std::{
    fmt,
    error,
    ops,
};

/// A struct to handle positive `f64` numbers.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PositiveF64(f64);

impl PositiveF64 {
    /// Generates a new `PositiveF64`.
    /// 
    /// The function returns a `Result<PositiveF64, InvalidNumber>`, because
    /// if the number is negative, an `InvalidNumber::NegativeValue` error is returned
    pub fn new(number: f64) -> Result<Self, InvalidNumber> {
        if number >= 0.0 {
            Ok(PositiveF64(number))
        } else {
            Err(InvalidNumber::NegativeValue)
        }
    }
    
    /// Allows to create a `PositiveF64` without checking if the number is positive.
    #[allow(dead_code)]
    pub unsafe fn new_unchecked(number: f64) -> Self {
        PositiveF64(number)
    }

}

impl fmt::Display for PositiveF64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ops::Add for PositiveF64 {
    type Output = PositiveF64;

    fn add(self, other: Self) -> Self {
        PositiveF64(self.0 + other.0) // a + b, if a and b are both positive, is positive
    }
}

// TODO: cambia
impl ops::Sub for PositiveF64 {
    type Output = PositiveF64;

    fn sub(self, other: Self) -> Self {
        PositiveF64::new(self.0 - other.0).unwrap() // a - b, if a and b are both positive, is positive if a > b, or 0 if a = b
    }
}

impl ops::AddAssign for PositiveF64 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

// prob shouldn't be there
impl ops::SubAssign for PositiveF64 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= if let Ok(o) = PositiveF64::new(self.0 - other.0) {
            o.0
        } else {
            0.0
        }
    }
}

/// An enum to handle invalid `PositiveF64` numbers.
#[derive(Debug)]
pub enum InvalidNumber {
    NegativeValue,
}

impl fmt::Display for InvalidNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
        	Self::NegativeValue => write!(f, "The number can't be negative."),
        }
    }
}

impl error::Error for InvalidNumber {}
