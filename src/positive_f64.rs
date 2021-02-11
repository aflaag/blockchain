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
    /// if the number is negative, an `InvalidNumber::NegativeValue` error is returned.
    /// 
    /// # Example
    /// ```
    /// # use blockchain::positive_f64::PositiveF64;
    /// let positive_f64 = PositiveF64::new(3.0).unwrap();
    /// 
    /// assert_eq!(positive_f64.value(), 3.0); // this method returns the value
    /// ```
    pub fn new(number: f64) -> Result<Self, InvalidNumber> {
        if number >= 0.0 {
            Ok(PositiveF64(number))
        } else {
            Err(InvalidNumber::NegativeValue)
        }
    }

    /// This method returns the value of the number, since the value in the struct isn't `pub`.
    /// 
    /// # Example
    /// ```
    /// # use blockchain::positive_f64::PositiveF64;
    /// let number = PositiveF64::new(5.0).unwrap();
    /// 
    /// assert_eq!(number.value(), 5.0);
    /// ```
    pub fn value(&self) -> f64 {
        self.0
    }
    
    /// Creates a new `PositiveF64` without checking the input.
    /// 
    /// # Safety
    /// Allows to create a `PositiveF64` without checking if the number is positive or `0.0`.
    /// 
    /// # Examples
    /// ```
    /// # use blockchain::positive_f64::PositiveF64;
    /// unsafe {
    ///     let number = PositiveF64::new_unchecked(4.0);
    /// 
    ///     assert_eq!(number.value(), 4.0);
    /// }
    /// ```
    /// 
    /// # Panics
    /// The invalid value could lead to uncertain behaviour.
    /// 
    /// ```
    /// # use blockchain::positive_f64::PositiveF64;
    /// unsafe {
    ///     let invalid_value = PositiveF64::new_unchecked(-9.0);
    /// 
    ///     // The above expression could make the program panic!
    /// }
    /// ```
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
        PositiveF64(self.0 + other.0)
    }
}

impl ops::Sub for PositiveF64 {
    type Output = PositiveF64;

    fn sub(self, other: Self) -> Self {
        let _ = PositiveF64::new(self.0 - other.0).unwrap(); // if the difference is >= 0.0

        PositiveF64::new(self.0 - other.0).unwrap()
    }
}

impl ops::AddAssign for PositiveF64 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl ops::SubAssign for PositiveF64 {
    fn sub_assign(&mut self, other: Self) {
        let _ = PositiveF64::new(self.0 - other.0).unwrap(); // if the difference is >= 0.0

        self.0 -= other.0;
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
